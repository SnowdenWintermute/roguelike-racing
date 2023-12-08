use crate::store::game_store::GameStore;
use common::{
    combatants::abilities::get_combatant_ability_attributes::TargetingScheme,
    dungeon_rooms::DungeonRoom,
    errors::AppError,
    packets::{
        client_to_server::ClientChangeTargetsPacket,
        server_to_client::CharacterSelectedAbilityPacket,
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
        target_ids_option,
    } = packet;

    let character = game_store.get_mut_character(character_id)?;
    character.combatant_properties.selected_ability_name = ability_name_option;
    character.combatant_properties.ability_target_ids = target_ids_option;
    Ok(())
}

pub fn handle_character_changed_targets(
    game_store: &mut GameStore,
    packet: ClientChangeTargetsPacket,
) -> Result<(), AppError> {
    let ClientChangeTargetsPacket {
        character_id,
        target_ids,
    } = packet;
    let character = game_store.get_mut_character(character_id)?;
    character.combatant_properties.ability_target_ids = Some(target_ids.clone());
    let selected_ability_name_option = character.combatant_properties.selected_ability_name.clone();
    if let Some(ability_name) = selected_ability_name_option {
        let ability_option = character
            .combatant_properties
            .abilities
            .get_mut(&ability_name);
        if let Some(ability) = ability_option {
            match ability.selected_targeting_scheme {
                TargetingScheme::Single => ability.most_recently_targeted_single = Some(target_ids),
                TargetingScheme::Area => ability.most_recently_targeted_area = Some(target_ids),
            }
        }
    }
    Ok(())
}
