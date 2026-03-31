use std::collections::HashMap;

use axum::extract::ws::Message;
use futures::channel::mpsc;
use tracing::{debug, error, info, warn};

use crate::models::{
    game::{Game, GameState},
    message::ServerMessage,
};

#[derive(Clone)]
pub struct Room {
    pub id: String,
    pub clients: HashMap<String, mpsc::UnboundedSender<Message>>,
    pub game: Game,
}

impl Room {
    pub fn new(id: String) -> Self {
        info!(room_id = %id, "room_created");
        Self {
            id,
            clients: HashMap::new(),
            game: Game::new(),
        }
    }

    // ───────────── CLIENT MANAGEMENT ─────────────

    pub fn add_client(&mut self, client_id: &str, client: mpsc::UnboundedSender<Message>) {
        self.clients.insert(client_id.into(), client);
        info!(
            room_id = %self.id,
            client_id = %client_id,
            total_clients = self.clients.len(),
            "client_joined"
        );
    }

    pub fn remove_client(&mut self, client_id: &str) {
        if self.clients.remove(client_id).is_some() {
            info!(
                room_id = %self.id,
                client_id = %client_id,
                total_clients = self.clients.len(),
                "client_left"
            );
        } else {
            warn!(room_id = %self.id, client_id = %client_id, "remove_nonexistent_client");
        }
    }

    pub fn broadcast(&mut self, msg: &Message, exclude_client: Option<&str>) {
        let before = self.clients.len();

        self.clients.retain(|client_id, sender| {
            if exclude_client.map_or(false, |ex| ex == client_id) {
                return true; // skip excluded client
            }

            match sender.unbounded_send(msg.clone()) {
                Ok(_) => true,
                Err(err) => {
                    warn!(
                        room_id = %self.id,
                        client_id = %client_id,
                        error = %err,
                        "client_disconnected_during_broadcast"
                    );
                    false
                }
            }
        });

        debug!(
            room_id = %self.id,
            attempted = before,
            delivered = self.clients.len(),
            dropped = before.saturating_sub(self.clients.len()),
            "broadcast_complete"
        );
    }

    pub fn broadcast_msg(&mut self, server_msg: &ServerMessage, exclude_client: Option<&str>) {
        match serde_json::to_string(server_msg) {
            Ok(json) => self.broadcast(&Message::Text(json.into()), exclude_client),
            Err(err) => error!(
                room_id = %self.id,
                error = %err,
                "failed_to_serialize_server_message"
            ),
        }
    }

    pub fn broadcast_game(&mut self) {
        self.broadcast_msg(&ServerMessage::Game(self.game.clone()), None);
    }

    pub fn broadcast_canvas(&mut self) {
        self.broadcast_msg(&ServerMessage::Canvas(self.game.canvas.clone()), None);
    }

    pub fn tick(&mut self) {
        let now = current_time_secs();

        match self.game.state {
            GameState::Waiting => {
                debug!(room_id = %self.id, total_players = self.clients.len(), "state_waiting");

                if self.clients.len() >= 2 {
                    info!(room_id = %self.id, total_players = self.clients.len(), "starting_game_from_waiting");
                    self.start_game();
                }
            }

            GameState::Playing => {
                if let Some(end_time) = self.game.round_end_time {
                    debug!(
                        room_id = %self.id,
                        now,
                        end_time,
                        remaining = end_time.saturating_sub(now),
                        "state_playing_tick"
                    );

                    if now >= end_time {
                        info!(room_id = %self.id, "round_finished");
                        self.game.stop(&now);
                        self.broadcast_game();
                    }
                }
            }

            GameState::GameOver => {
                if let Some(end_time) = self.game.round_end_time {
                    debug!(room_id = %self.id, now, end_time, "state_gameover_tick");

                    if now >= end_time {
                        if self.clients.len() >= 2 {
                            info!(room_id = %self.id, total_players = self.clients.len(), "restarting_game");
                            self.start_game();
                        } else {
                            info!(room_id = %self.id, total_players = self.clients.len(), "not_enough_players_waiting");
                            self.game.wait();
                            self.broadcast_game();
                        }
                    }
                }
            }
        }
    }

    pub fn start_game(&mut self) {
        if self.clients.len() < 2 {
            warn!(room_id = %self.id, total_players = self.clients.len(), "start_game_rejected_not_enough_players");
            return;
        }

        let drawer = self.clients.keys().next().unwrap().clone();
        let now = current_time_secs();
        let end_time = now + 60;

        info!(
            room_id = %self.id,
            drawer = %drawer,
            total_players = self.clients.len(),
            start_time = now,
            end_time,
            duration = 60,
            "game_started"
        );

        self.game.start(&drawer, &end_time);
        self.broadcast_game();
    }
}

#[inline]
fn current_time_secs() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs()
}
