use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Game {
    canvas: Canvas,
    word: String,
    drawer: String,
}

impl Game {
    pub fn new() -> Self {
        Self {
            canvas: Canvas::new(),
            word: String::new(),
            drawer: String::new(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Canvas {
    width: u32,
    height: u32,
    pixels: Vec<Pixel>,
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
    x: u32,
    y: u32,
    color: String,
}

