use crate::adventuring_party::AdventuringParty;
use crate::character::Character;
use crate::combatants::CombatantClass;
use crate::errors::AppError;
use crate::game::RoguelikeRacerGame;
use serde::Deserialize;
use serde::Serialize;
use std::time::Instant;

#[derive(Debug, Serialize, Deserialize)]
pub enum GameServerUpdatePackets {
    // ERROR
    Error(String),
    // USER
    ClientUserName(String),
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
    AdventuringPartyCreated(AdventuringPartyCreation),
    AdventuringPartyRemoved(u32),
    PlayerChangedAdventuringParty(PlayerAdventuringPartyChange),
    ClientAdventuringPartyId(Option<u32>),
    CharacterCreation(NewCharacterInParty),
    CharacterClassSelection(PlayerCharacterClassSelection),
    CharacterNameChange(PlayerCharacterNameChange),
    CharacterDeletion(PlayerCharacterDeletion),
    PlayerToggledReady(String),
    GameStarted(u128),
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
pub struct RoomState {
    pub room_name: String,
    pub users: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct GameListEntry {
    pub game_name: String,
    pub number_of_users: u8,
    pub time_started: Option<u128>,
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
    pub party_id: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PlayerRemovedFromGame {
    pub username: String,
    pub game_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct NewCharacterInParty {
    pub party_id: u32,
    pub username: String,
    pub character_id: u32,
    pub character_name: String,
    pub combatant_class: CombatantClass,
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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AdventuringPartyCreation {
    pub party_id: u32,
    pub party_name: String,
    pub username_created_by: String,
}
