use common::packets::server_to_client::{GameListEntry, RoomState};
use yewdux::prelude::*;

#[derive(Store, Default, PartialEq, Clone)]
pub struct LobbyStore {
    pub game_list: Vec<GameListEntry>,
    pub room: RoomState,
}
