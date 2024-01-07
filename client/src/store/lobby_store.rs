use common::packets::server_to_client::GameListEntry;
use yewdux::prelude::*;

#[derive(Store, Default, PartialEq, Clone)]
pub struct LobbyStore {
    pub username: String,
    pub game_list: Vec<GameListEntry>,
}
