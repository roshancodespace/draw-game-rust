use axum::{
    extract::{
        State, WebSocketUpgrade,
        ws::{Message, WebSocket},
    },
    response::IntoResponse,
};
use futures::{SinkExt, StreamExt, channel::mpsc};
use tracing::error;

use crate::{
    models::{
        game::{GameState, Pixel},
        message::{ClientMessage, ServerMessage},
    },
    state::AppState,
};

pub async fn ws_handler(State(state): State<AppState>, ws: WebSocketUpgrade) -> impl IntoResponse {
    ws.on_upgrade(move |socket: WebSocket| handle_socket(state, socket))
}

async fn handle_socket(state: AppState, socket: WebSocket) {
    let (mut ws_sender, mut ws_receiver) = socket.split();

    let (tx, mut rx) = mpsc::unbounded();

    let client_id = uuid::Uuid::new_v4().to_string();
    let room = state.join_available_room(&client_id, tx);

    {
        let ws_msg = ServerMessage::Game(room.game.clone());
        let json = serde_json::to_string(&ws_msg).unwrap();

        if ws_sender.send(Message::Text(json.into())).await.is_err() {
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

    let client_id_clone = client_id.clone();
    let mut room_clone = room.clone();
    let state_clone = state.clone();

    let mut recv_task = tokio::spawn(async move {
        while let Some(Ok(msg)) = ws_receiver.next().await {
            match msg {
                Message::Text(text) => match serde_json::from_str::<ClientMessage>(&text) {
                    Ok(client_msg) => match client_msg {
                        ClientMessage::Draw { x, y, color } => {
                            if (room_clone.game.state == GameState::Playing) {
                                room_clone.game.canvas.paint(Pixel { x, y, color });
                                room_clone.broadcast_canvas();
                            }
                        }
                        ClientMessage::Chat { sender, message } => {
                            let server_msg = ServerMessage::Chat { sender, message };
                            let outgoing = serde_json::to_string(&server_msg).unwrap();

                            state_clone.broadcast_to_room(
                                &room_clone.id,
                                &Message::Text(outgoing.into()),
                                Some(&client_id_clone),
                            );
                        }
                    },

                    Err(e) => {
                        error!(error = %e, "Invalid message");
                    }
                },

                Message::Close(_) => break,

                _ => {}
            }
        }
    });

    // whichever task finishes first cancels the other
    tokio::select! {
        _ = (&mut send_task) => recv_task.abort(),
        _ = (&mut recv_task) => send_task.abort(),
    }

    // cleanup
    state.remove_client_from_room(&room.id, &client_id);
}
