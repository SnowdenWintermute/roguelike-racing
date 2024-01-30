use crate::components::mesh_manager::CombatantEventManager;
use crate::store::game_store::GameStore;
use common::app_consts::error_messages;
use common::combat::battle::Battle;
use common::dungeon_rooms::DungeonRoom;
use common::dungeon_rooms::DungeonRoomTypes;
use common::errors::AppError;
use common::game::getters::get_mut_party;
use common::packets::client_to_server::ChangeTargetsPacket;
use common::packets::server_to_client::CharacterSelectedAbilityPacket;
use common::packets::server_to_client::CharacterSelectedConsumablePacket;
use gloo::console::log;
use std::collections::HashMap;
use std::collections::VecDeque;
use yewdux::Dispatch;

pub fn new_dungeon_room_types_on_current_floor(
    game_dispatch: Dispatch<GameStore>,
    packet: VecDeque<Option<DungeonRoomTypes>>,
) -> Result<(), AppError> {
    log!("got new room types list");
    game_dispatch.reduce_mut(|store| {
        let party = store.get_current_party_mut()?;
        party.client_current_floor_rooms_list = packet;
        party.rooms_explored.on_current_floor = 0;
        Ok(())
    })
}

pub fn handle_player_toggled_ready_to_explore(
    game_dispatch: Dispatch<GameStore>,
    username: String,
) -> Result<(), AppError> {
    log!("player toggled ready to explore");
    game_dispatch.reduce_mut(|store| -> Result<(), AppError> {
        let party = store.get_current_party_mut()?;
        if party.players_ready_to_descend.contains(&username) {
            party.players_ready_to_descend.remove(&username);
        }
        if party.players_ready_to_explore.contains(&username) {
            party.players_ready_to_explore.remove(&username);
        } else {
            party.players_ready_to_explore.insert(username.clone());
        };
        Ok(())
    })
}

pub fn handle_player_toggled_ready_to_descend(
    game_dispatch: Dispatch<GameStore>,
    username: String,
) -> Result<(), AppError> {
    game_dispatch.reduce_mut(|store| -> Result<(), AppError> {
        let party = store.get_current_party_mut()?;
        if party.players_ready_to_explore.contains(&username) {
            party.players_ready_to_explore.remove(&username);
        }
        if party.players_ready_to_descend.contains(&username) {
            party.players_ready_to_descend.remove(&username);
        } else {
            party.players_ready_to_descend.insert(username.clone());
        };
        Ok(())
    })
}

pub fn handle_new_dungeon_room(
    game_store: &mut GameStore,
    packet: DungeonRoom,
) -> Result<(), AppError> {
    log!("new dungeon room");
    if let Some(monsters) = &packet.monsters {
        for (monster_id, _) in monsters {
            game_store
                .action_results_manager
                .combantant_event_managers
                .insert(*monster_id, CombatantEventManager::new(*monster_id));
        }
    }
    let party = game_store.get_current_party_mut()?;
    party.players_ready_to_explore.clear();
    party.players_ready_to_descend.clear();
    let current_room_type = packet.room_type;
    party.current_room = packet;
    party.rooms_explored.on_current_floor += 1;
    let num_rooms_explored_on_current_floor = party.rooms_explored.on_current_floor;
    party.rooms_explored.total += 1;
    let room_to_reveal = party
        .client_current_floor_rooms_list
        .get_mut((num_rooms_explored_on_current_floor - 1).into())
        .ok_or_else(|| AppError {
            error_type: common::errors::AppErrorTypes::ClientError,
            message: error_messages::CLIENT_LIST_MISSING_ROOM_TYPE.to_string(),
        })?;
    *room_to_reveal = Some(current_room_type);

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
    character.combatant_properties.combat_action_targets = targets_option;
    Ok(())
}

pub fn handle_character_consumable_selection(
    game_dispatch: Dispatch<GameStore>,
    packet: CharacterSelectedConsumablePacket,
) -> Result<(), AppError> {
    log!(format!(
        "selected consumable packet received {:#?}",
        &packet
    ));
    let CharacterSelectedConsumablePacket {
        character_id,
        targets_option,
        consumable_id_option,
    } = packet;

    game_dispatch.reduce_mut(|store| {
        let character = store.get_mut_character(character_id)?;
        character.combatant_properties.selected_consumable = consumable_id_option;
        character.combatant_properties.combat_action_targets = targets_option;
        Ok(())
    })
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
    character.combatant_properties.combat_action_targets = Some(new_targets.clone());

    Ok(())
}
