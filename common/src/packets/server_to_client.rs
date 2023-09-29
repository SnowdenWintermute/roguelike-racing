use crate::game::RoguelikeRacerGame;
use serde::{Deserialize, Serialize};
use std::time::Instant;

#[derive(Debug, Serialize, Deserialize)]
pub struct RoomFullUpdate {
    pub room_name: String,
    pub users: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GameListEntry {
    pub game_name: String,
    pub number_of_users: u8,
    pub time_started: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GameListFullUpdate {
    pub games: Vec<GameListEntry>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FullUpdate {
    pub room: RoomFullUpdate,
    pub game_list: GameListFullUpdate,
    pub current_game: Option<RoguelikeRacerGame>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum GameServerUpdatePackets {
    FullUpdate(FullUpdate),
}
