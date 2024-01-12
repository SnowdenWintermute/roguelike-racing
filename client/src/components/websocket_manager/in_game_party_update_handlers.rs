use crate::components::mesh_manager::CombatantEventManager;
use crate::store::game_store::GameStore;
use common::app_consts::error_messages;
use common::combat::battle::Battle;
use common::dungeon_rooms::DungeonRoom;
use common::errors::AppError;
use common::game::getters::get_mut_party;
use common::packets::client_to_server::ChangeTargetsPacket;
use common::packets::server_to_client::CharacterSelectedAbilityPacket;
use gloo::console::log;
use std::collections::HashMap;
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
    if let Some(monsters) = &packet.monsters {
        for (monster_id, _) in monsters {
            game_store
                .action_results_manager
                .combantant_event_managers
                .insert(*monster_id, CombatantEventManager::new(*monster_id));
        }
    }
    let party = game_store.get_current_party_mut()?;
    party.players_ready_to_explore = HashSet::new();
    party.current_room = packet;
    party.rooms_explored.on_current_floor += 1;
    party.rooms_explored.total += 1;

    Ok(())
}

pub fn handle_battle_full_update(
    game_store: &mut GameStore,
    battle_option: Option<Battle>,
) -> Result<(), AppError> {
    let game = game_store.game.as_mut().ok_or_else(|| AppError {
        error_type: common::errors::AppErrorTypes::ClientError,
        message: error_messages::GAME_NOT_FOUND.to_string(),
    })?;
    if let Some(battle) = battle_option {
        game_store.current_battle_id = Some(battle.id);
        if let Some(party_id) = game_store.current_party_id {
            let party = get_mut_party(game, party_id)?;
            party.battle_id = Some(battle.id);
        }

        game.battles.insert(battle.id, battle);
    } else {
        game_store.current_battle_id = None;
        game.battles = HashMap::new();
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
