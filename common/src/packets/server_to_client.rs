use crate::game::RoguelikeRacerGame;
use serde::{Deserialize, Serialize};
use std::time::Instant;

#[derive(Debug, Serialize, Deserialize)]
pub struct RoomState {
    pub room_name: String,
    pub users: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct GameListEntry {
    pub game_name: String,
    pub number_of_users: u8,
    pub time_started: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ClientGameListState {
    pub games: Vec<GameListEntry>,
}

impl ClientGameListState {
    pub fn new() -> Self {
        ClientGameListState { games: Vec::new() }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RoguelikeRacerAppState {
    pub room: RoomState,
    pub game_list: ClientGameListState,
    pub current_game: Option<RoguelikeRacerGame>,
}

impl RoguelikeRacerAppState {
    pub fn new() -> Self {
        RoguelikeRacerAppState {
            room: RoomState {
                room_name: "".to_string(),
                users: Vec::new(),
            },
            game_list: ClientGameListState { games: Vec::new() },
            current_game: None,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum GameServerUpdatePackets {
    FullUpdate(RoguelikeRacerAppState),
    GameList(ClientGameListState),
    GameFullUpdate(Option<RoguelikeRacerGame>),
    GamePlayerJoined(String),
    GamePlayerLeft(String),
    RoomFullUpdate(RoomState),
    UserJoinedRoom(String),
    UserLeftRoom(String),
}
