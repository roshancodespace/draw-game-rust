use serde::{Deserialize, Serialize};

use crate::models::{
    chat::Chat,
    game::{Canvas, Game},
};

#[derive(Serialize, Deserialize, Clone)]
pub enum WsMessage {
    Game(Game),
    Chat(Chat),
    Canvas(Canvas),
}
