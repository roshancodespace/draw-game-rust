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
}
