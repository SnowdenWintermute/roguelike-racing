use super::client_to_server::ChangeTargetsPacket;
use super::client_to_server::UnequipSlotRequest;
use super::WebsocketChannelNamespace;
use crate::app_consts::LOBBY_CHANNEL;
use crate::character::Character;
use crate::combat::battle::Battle;
use crate::combat::ActionResult;
use crate::combat::CombatTurnResult;
use crate::combatants::abilities::AbilityTarget;
use crate::combatants::abilities::CombatantAbilityNames;
use crate::combatants::CombatantClass;
use crate::dungeon_rooms::DungeonRoom;
use crate::game::RoguelikeRacerGame;
use crate::items::Item;
use serde::Deserialize;
use serde::Serialize;
use std::collections::HashSet;

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
    // CHANNELS
    WebsocketChannelFullUpdate(WebsocketChannelFullState),
    UserJoinedWebsocketChannel(WebsocketChannelAndUserPacket),
    UserLeftWebsocketChannel(WebsocketChannelAndUserPacket),
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
    // IN GAME
    CharacterEquippedItem(CharacterEquippedItemPacket),
    CharacterUnequippedSlot(UnequipSlotRequest),
    PlayerToggledReadyToExplore(String),
    DungeonRoomUpdate(DungeonRoom),
    CharacterSelectedAbility(CharacterSelectedAbilityPacket),
    CharacterChangedTargets(ChangeTargetsPacket),
    ActionResults(Vec<ActionResult>),
    CombatTurnResults(CombatTurnResultsPacket),
    BattleFullUpdate(Option<Battle>),
    BattleVictory(Option<Vec<Item>>),
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
pub struct WebsocketChannelFullState {
    pub name: String,
    pub namespace: WebsocketChannelNamespace,
    pub usernames_in_channel: HashSet<String>,
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

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
pub struct WebsocketChannelsState {
    pub main: WebsocketChannelFullState,
    pub party: Option<WebsocketChannelFullState>,
    pub chat: Option<WebsocketChannelFullState>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RoguelikeRacerAppState {
    pub websocket_channels: WebsocketChannelsState,
    pub game_list: ClientGameListState,
    pub current_game: Option<RoguelikeRacerGame>,
}

impl RoguelikeRacerAppState {
    pub fn new() -> Self {
        RoguelikeRacerAppState {
            websocket_channels: WebsocketChannelsState {
                main: WebsocketChannelFullState {
                    name: LOBBY_CHANNEL.to_string(),
                    namespace: WebsocketChannelNamespace::Lobby,
                    usernames_in_channel: HashSet::new(),
                },
                party: None,
                chat: None,
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
    pub character: Character,
    pub username: String,
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
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct CharacterEquippedItemPacket {
    pub character_id: u32,
    pub item_id: u32,
    pub alt_slot: bool,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct CharacterSelectedAbilityPacket {
    pub character_id: u32,
    pub ability_name_option: Option<CombatantAbilityNames>,
    pub targets_option: Option<AbilityTarget>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct CombatTurnResultsPacket {
    pub turn_results: Vec<CombatTurnResult>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct WebsocketChannelAndUserPacket {
    pub username: String,
    pub channel_name: String,
    pub channel_namespace: WebsocketChannelNamespace,
}
