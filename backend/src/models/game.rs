use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub enum GameState {
    Waiting,
    Playing,
    GameOver,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Game {
    pub state: GameState,
    pub round_end_time: Option<u64>,
    pub canvas: Canvas,
    pub word: String,
    pub drawer: String,
}

impl Game {
    pub fn new() -> Self {
        Self {
            state: GameState::Waiting,
            round_end_time: None,
            canvas: Canvas::new(),
            word: String::new(),
            drawer: String::new(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Canvas {
    pub width: u32,
    pub height: u32,
    pub pixels: Vec<Pixel>,
}

impl Canvas {
    pub fn new() -> Self {
        Self {
            width: 800,
            height: 600,
            pixels: Vec::new(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Pixel {
    pub x: u32,
    pub y: u32,
    pub color: String,
}

