use serde::Serialize;

#[derive(Serialize)]
pub struct RoomResponse {
    pub total: usize,
    pub rooms: Vec<RoomInfo>
}

#[derive(Serialize)]
pub struct RoomInfo {
    pub id: String,
    pub clients: usize,
}