use std::usize;

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

    pub fn stop(&mut self, now: &u64) {
        self.state = GameState::GameOver;
        self.round_end_time = Some(*now + 20);
    }

    pub fn start(&mut self, drawer: &str, end_time: &u64) {
        self.drawer = drawer.into();
        self.round_end_time = Some(*end_time);
        self.state = GameState::Playing;
    }

    pub fn wait(&mut self) {
        self.state = GameState::Waiting;
        self.round_end_time = None;
        self.drawer = String::new();
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Canvas {
    pub width: u64,
    pub height: u64,
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

    pub fn set_size(&mut self, width: u64, height: u64) {
        self.height = height;
        self.width = width;
    }

    pub fn paint(&mut self, pixel: Pixel) {
        self.pixels.retain(|p| p != &pixel);
        self.pixels.push(pixel.clone());
    }
}

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct Pixel {
    pub x: u32,
    pub y: u32,
    pub color: String,
}
