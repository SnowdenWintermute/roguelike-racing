use crate::{
    adventuring_party::AdventuringParty,
    character::{combatant_properties::CombatantClass, Character},
    game::RoguelikeRacerGame,
};
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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PlayerAdventuringPartyChange {
    pub username: String,
    pub party_id: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PlayerCharacterCreation {
    pub party_id: u32,
    pub username: String,
    pub class: CombatantClass,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PlayerCharacterClassSelection {
    pub character_id: u32,
    pub class: CombatantClass,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PlayerCharacterNameChange {
    pub character_id: u32,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PlayerCharacterDeletion {
    pub party_id: u32,
    pub username: String,
    pub character_id: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum GameServerUpdatePackets {
    // FULL STATE UPDATES
    FullUpdate(RoguelikeRacerAppState),
    GameList(ClientGameListState),
    GameFullUpdate(Option<RoguelikeRacerGame>),
    RoomFullUpdate(RoomState),
    // ROOMS
    UserJoinedRoom(String),
    UserLeftRoom(String),
    // GAME IN LOBBY
    UserJoinedGame(String),
    UserLeftGame(String),
    AdventuringPartyCreated(AdventuringParty),
    AdventuringPartyRemoved(u32),
    PlayerChangedAdventuringParty(PlayerAdventuringPartyChange),
    ClientAdventuringPartyId(Option<u32>),
    CharacterCreation(PlayerCharacterCreation),
    CharacterClassSelection(PlayerCharacterClassSelection),
    CharacterNameChange(PlayerCharacterNameChange),
    CharacterDeletion(PlayerCharacterDeletion),
    PlayerToggledReady(String),
}
