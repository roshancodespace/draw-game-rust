use axum::{
    extract::{
        State, WebSocketUpgrade,
        ws::{Message, WebSocket},
    },
    response::IntoResponse,
};
use futures::{SinkExt, StreamExt, channel::mpsc};
use tracing::info;

use crate::state::AppState;

pub async fn ws_handler(State(state): State<AppState>, ws: WebSocketUpgrade) -> impl IntoResponse {
    ws.on_upgrade(move |socket: WebSocket| handle_socket(state, socket))
}

async fn handle_socket(state: AppState, socket: WebSocket) {
    let (mut ws_sender, mut ws_receiver) = socket.split();
    let (tx, mut rx) = mpsc::unbounded();

    let client_id = uuid::Uuid::new_v4().to_string();
    let room = state.join_available_room(client_id.clone(), tx);

    {
        info!(client_id = %client_id, room_id = %room.id, "Client joined room");
        let json = serde_json::to_string(&room.game).unwrap().into();
        if ws_sender.send(Message::Text(json)).await.is_err() {
            return;
        }
    }

    let mut send_task = tokio::spawn(async move {
        while let Some(msg) = rx.next().await {
            if ws_sender.send(msg).await.is_err() {
                break;
            }
        }
    });

    let room_id_clone = room.id.clone();
    let state_clone = state.clone();
    let mut recv_task = tokio::spawn(async move {
        while let Some(Ok(msg)) = ws_receiver.next().await {
            match msg {
                Message::Close(_) => break,
                Message::Text(_) | Message::Binary(_) => {
                    state_clone.broadcast_to_room(&room_id_clone, msg);
                }
                _ => {}
            }
        }
    });

    tokio::select! {
        _ = (&mut send_task) => recv_task.abort(),
        _ = (&mut recv_task) => send_task.abort(),
    }

    state.remove_client_from_room(&room.id, &client_id);
    info!(client_id = %client_id, room_id = %room.id, "Client disconnected from room");
}
