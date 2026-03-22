use axum::{Router, routing::get};

use crate::{
    handlers::{room_handler, ws_handler},
    state::AppState,
};

pub fn get_router(app_state: AppState) -> Router {
    Router::new()
        .route("/ws", get(ws_handler))
        .route("/rooms", get(room_handler))
        .with_state(app_state)
}
