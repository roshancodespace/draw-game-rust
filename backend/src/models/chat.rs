use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Chat {
    pub sender: String,
    pub message: ChatMessage,
}

#[derive(Serialize, Deserialize, Clone)]
pub enum ChatMessage {
    Text(String),
    Guess(String),
    CorrectGuess(String),
    System(String),
}

