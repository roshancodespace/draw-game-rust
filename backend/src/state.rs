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
        Self {
            rooms: Arc::new(RwLock::new(Vec::new())),
        }
    }

    pub fn join_available_room(
        &self,
        client_id: String,
        client_sender: mpsc::UnboundedSender<Message>,
    ) -> Room {
        let mut rooms = self.rooms.write().unwrap();

        for room in rooms.iter_mut() {
            if room.clients.len() < MAX_CAPACITY {
                room.add_client(client_id.clone(), client_sender);
                return room.clone();
            }
        }

        let new_room_id = uuid::Uuid::new_v4().to_string();
        let mut new_room = Room::new(new_room_id.clone());

        new_room.add_client(client_id.clone(), client_sender);
        rooms.push(new_room.clone());

        new_room
    }

    pub fn broadcast_to_room(&self, room_id: &str, msg: Message) {
        let mut rooms = self.rooms.write().unwrap();
        if let Some(room) = rooms.iter_mut().find(|r| r.id == room_id) {
            room.broadcast(msg);
        }
    }

    pub fn remove_client_from_room(&self, room_id: &str, client_id: &str) {
        let mut rooms = self.rooms.write().unwrap();
        if let Some(room) = rooms.iter_mut().find(|r| r.id == room_id) {
            room.remove_client(client_id);
        }
    }
}
