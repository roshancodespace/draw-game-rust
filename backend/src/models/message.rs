use serde::{Deserialize, Serialize};

use crate::models::game::{Canvas, Game};

#[derive(Serialize, Deserialize, Clone)]
pub enum ServerMessage {
    Game(Game),
    Chat { sender: String, message: String },
    Canvas(Canvas),
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(tag = "type", content = "data")]
pub enum ClientMessage {
    Draw { x: u32, y: u32, color: String },
    Chat { sender: String, message: String },
}
