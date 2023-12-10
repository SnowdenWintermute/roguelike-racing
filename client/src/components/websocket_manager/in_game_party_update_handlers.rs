use crate::store::game_store::GameStore;
use common::{
    dungeon_rooms::DungeonRoom,
    errors::AppError,
    packets::{
        client_to_server::ChangeTargetsPacket, server_to_client::CharacterSelectedAbilityPacket,
    },
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
    party.rooms_explored.on_current_floor += 1;
    party.rooms_explored.total += 1;
    if party.current_room.monsters.is_some() {
        let turn_trackers = party.get_combat_turn_order();
        party.combatant_turn_trackers = Some(turn_trackers);
    }

    Ok(())
}

pub fn handle_character_ability_selection(
    game_store: &mut GameStore,
    packet: CharacterSelectedAbilityPacket,
) -> Result<(), AppError> {
    let CharacterSelectedAbilityPacket {
        character_id,
        ability_name_option,
        targets_option,
    } = packet;

    let character = game_store.get_mut_character(character_id)?;
    character.combatant_properties.selected_ability_name = ability_name_option;
    character.combatant_properties.ability_targets = targets_option;
    Ok(())
}

pub fn handle_character_changed_targets(
    game_store: &mut GameStore,
    packet: ChangeTargetsPacket,
) -> Result<(), AppError> {
    let ChangeTargetsPacket {
        character_id,
        new_targets,
    } = packet;
    let character = game_store.get_mut_character(character_id)?;
    character.combatant_properties.ability_targets = Some(new_targets.clone());

    Ok(())
}
