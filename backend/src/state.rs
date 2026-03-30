use std::sync::{Arc, RwLock};

use axum::extract::ws::Message;
use futures::channel::mpsc;

use crate::models::room::Room;

const MAX_CAPACITY: usize = 4;

#[derive(Clone)]
pub struct AppState {
    pub rooms: Arc<RwLock<Vec<Room>>>,
}

impl AppState {
    pub fn new() -> Self {
        let state = Self {
            rooms: Arc::new(RwLock::new(Vec::new())),
        };

        let state_clone = state.clone();
        tokio::spawn(async move {
            loop {
                tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
                state_clone.tick();
            }
        });

        state
    }

    pub fn tick(&self) {
        let mut rooms = self.rooms.write().unwrap();
        for room in rooms.iter_mut() {
            room.tick();
        }
    }

    pub fn join_available_room(
        &self,
        client_id: &str,
        client_sender: mpsc::UnboundedSender<Message>,
    ) -> Room {
        let mut rooms = self.rooms.write().unwrap();

        for room in rooms.iter_mut() {
            if room.clients.len() < MAX_CAPACITY {
                room.add_client(client_id, client_sender);
                return room.clone();
            }
        }

        let new_room_id = uuid::Uuid::new_v4().to_string();
        let mut new_room = Room::new(new_room_id);

        new_room.add_client(client_id, client_sender);
        rooms.push(new_room.clone());

        new_room
    }

    pub fn broadcast_to_room(&self, room_id: &str, msg: &Message, exclude_client: Option<&str>) {
        let mut rooms = self.rooms.write().unwrap();
        if let Some(room) = rooms.iter_mut().find(|r| r.id == room_id) {
            room.broadcast(&msg, exclude_client);
        }
    }

    pub fn remove_client_from_room(&self, room_id: &str, client_id: &str) {
        let mut rooms = self.rooms.write().unwrap();
        if let Some(room) = rooms.iter_mut().find(|r| r.id == room_id) {
            room.remove_client(client_id);
            room.tick();
        }
    }
}
