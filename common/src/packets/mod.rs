use crate::items::equipment::EquipmentSlots;
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
