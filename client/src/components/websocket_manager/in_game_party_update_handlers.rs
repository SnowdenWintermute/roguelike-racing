use crate::store::game_store::GameStore;
use common::{
    dungeon_rooms::DungeonRoom, errors::AppError,
    packets::server_to_client::CharacterSelectedAbilityPacket,
};
use gloo::console::log;
use std::collections::HashSet;

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

pub fn handle_character_ability_selection(
    game_store: &mut GameStore,
    packet: CharacterSelectedAbilityPacket,
) -> Result<(), AppError> {
    let CharacterSelectedAbilityPacket {
        character_id,
        ability_name_option,
        target_ids_option,
    } = packet;

    let character = game_store.get_mut_character(character_id)?;
    character.combatant_properties.selected_ability_name = ability_name_option;
    character.combatant_properties.ability_target_ids = target_ids_option;
    Ok(())
}
