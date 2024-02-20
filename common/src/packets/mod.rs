use crate::items::equipment::EquipmentSlots;
use crate::primatives::NextOrPrevious;
use serde::Deserialize;
use serde::Serialize;
pub mod client_to_server;
pub mod server_to_client;

#[derive(Debug, Serialize, Deserialize, Hash, Clone, PartialEq, Default, Eq)]
pub enum WebsocketChannelNamespace {
    #[default]
    Lobby,
    Game,
    Party,
    Chat,
}

pub type CharacterId = u32;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct CharacterAndItem {
    pub character_id: u32,
    pub item_id: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CharacterAndSlot {
    pub character_id: u32,
    pub slot: EquipmentSlots,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct CharacterAndDirection {
    pub character_id: u32,
    pub direction: NextOrPrevious,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum GameMessages {
    PartyDescent(String, u8),
    PartyEscape(String, u64),
    PartyWipe(String, u8, u64),
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct ExperienceChange {
    pub combatant_id: u32,
    pub experience_change: i16,
}
