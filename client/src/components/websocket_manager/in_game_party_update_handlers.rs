use std::collections::HashSet;

use crate::store::game_store::GameStore;
use common::{dungeon_rooms::DungeonRoom, errors::AppError};
use gloo::console::log;

pub fn handle_player_toggled_ready_to_explore(
    game_store: &mut GameStore,
    username: String,
) -> Result<(), AppError> {
    let party = game_store.get_current_party_mut()?;

    if party.players_ready_to_explore.contains(&username) {
        party.players_ready_to_explore.remove(&username);
    } else {
        party.players_ready_to_explore.insert(username.clone());
    };

    log!(format!("player {} toggled ready to explore", username));

    Ok(())
}

pub fn handle_new_dungeon_room(
    game_store: &mut GameStore,
    packet: DungeonRoom,
) -> Result<(), AppError> {
    let party = game_store.get_current_party_mut()?;
    party.players_ready_to_explore = HashSet::new();
    party.current_room = packet;

    Ok(())
}
