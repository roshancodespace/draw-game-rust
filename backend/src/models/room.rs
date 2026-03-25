use std::collections::HashMap;

use axum::extract::ws::Message;
use futures::channel::mpsc;
use tracing::warn;

use crate::models::game::Game;

#[derive(Clone)]
pub struct Room {
    pub id: String,
    pub clients: HashMap<String, mpsc::UnboundedSender<Message>>,
    pub game: Game,
}

impl Room {
    pub fn new(id: String) -> Self {
        Self {
            id,
            clients: HashMap::new(),
            game: Game::new(),
        }
    }

    pub fn add_client(&mut self, client_id: String, client: mpsc::UnboundedSender<Message>) {
        self.clients.insert(client_id, client);
    }

    pub fn remove_client(&mut self, client_id: &str) {
        self.clients.remove(client_id);
    }

    pub fn broadcast(&mut self, msg: Message) {
        self.clients.retain(|client_id, sender| {
            match sender.unbounded_send(msg.clone()) {
                Ok(_) => true,
                Err(err) => {
                    warn!(client_id = %client_id, error = %err, "Removing dead client");
                    false
                }
            }
        });
    }

    pub fn tick(&mut self) {
        let now = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs();

        match self.game.state {
            crate::models::game::GameState::Waiting => {
                if self.clients.len() >= 2 {
                    self.start_game();
                }
            }
            crate::models::game::GameState::Playing => {
                if let Some(end_time) = self.game.round_end_time {
                    if now >= end_time {
                        self.game.state = crate::models::game::GameState::GameOver;
                        self.game.round_end_time = Some(now + 20); // 20s cooldown
                        self.broadcast_state();
                    }
                }
            }
            crate::models::game::GameState::GameOver => {
                if let Some(end_time) = self.game.round_end_time {
                    if now >= end_time {
                        if self.clients.len() >= 2 {
                            self.start_game();
                        } else {
                            self.game.state = crate::models::game::GameState::Waiting;
                            self.game.round_end_time = None;
                            self.broadcast_state();
                        }
                    }
                }
            }
        }
    }

    pub fn start_game(&mut self) {
        if self.clients.len() < 2 {
            return;
        }

        let drawer_id = self.clients.keys().next().unwrap().clone();
        self.game.drawer = drawer_id.clone();
        self.game.state = crate::models::game::GameState::Playing;

        let now = std::time::SystemTime::now();
        let end_time = now.duration_since(std::time::UNIX_EPOCH).unwrap().as_secs() + 60; // 60s round
        self.game.round_end_time = Some(end_time);

        self.broadcast_state();
    }

    pub fn broadcast_state(&mut self) {
        let msg = crate::models::message::WsMessage::Game(self.game.clone());
        if let Ok(json) = serde_json::to_string(&msg) {
            self.broadcast(Message::Text(json.into()));
        }
    }
}
