use axum::{Json, extract::State};

use crate::{models::response::{RoomInfo, RoomResponse}, state::AppState};

pub async fn room_handler(State(state): State<AppState>) -> Json<RoomResponse> {
    let rooms = state.rooms.read().unwrap();
    Json(RoomResponse { total: rooms.len(), rooms: rooms.iter().map(|room| RoomInfo { id: room.id.clone(), clients: room.clients.len() }).collect() })
}
