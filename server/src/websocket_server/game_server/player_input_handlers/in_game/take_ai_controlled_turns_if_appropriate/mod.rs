#![allow(unused_assignments)]
use crate::websocket_server::game_server::player_input_handlers::in_game::apply_action_results::apply_action_results;
use common::app_consts::error_messages;
use common::combat::CombatTurnResult;
use common::combatants::CombatantControlledBy;
use common::errors::AppError;
use common::game::RoguelikeRacerGame;

pub fn take_ai_controlled_turns_if_appropriate(
    game: &mut RoguelikeRacerGame,
    battle_id: u32,
) -> Result<Vec<CombatTurnResult>, AppError> {
    let mut ai_turn_results = vec![];
    let battle = game.battles.get(&battle_id).ok_or_else(|| AppError {
        error_type: common::errors::AppErrorTypes::Generic,
        message: error_messages::BATTLE_NOT_FOUND.to_string(),
    })?;
    let turn_trackers = &battle.combatant_turn_trackers;
    let mut active_combatant_id = turn_trackers
        .first()
        .ok_or_else(|| AppError {
            error_type: common::errors::AppErrorTypes::Generic,
            message: error_messages::TURN_TRACKERS_EMPTY.to_string(),
        })?
        .entity_id;

    let (mut active_combatant_entity_properties, mut active_combatant_properties) =
        game.get_combatant_by_id(&active_combatant_id)?;
    let mut active_combatant_is_ai_controlled =
        active_combatant_properties.controlled_by == CombatantControlledBy::AI;
    let mut active_combatant_turn_action_results = vec![];

    while active_combatant_is_ai_controlled {
        println!("taking AI turn");
        let battle = game.battles.get(&battle_id).ok_or_else(|| AppError {
            error_type: common::errors::AppErrorTypes::ServerError,
            message: error_messages::BATTLE_NOT_FOUND.to_string(),
        })?;
        let (ally_battle_group, enemy_battle_group) =
            battle.get_ally_and_enemy_battle_groups(&active_combatant_id)?;
        let enemy_ids = enemy_battle_group.combatant_ids.clone();
        let ally_ids = ally_battle_group.combatant_ids.clone();
        let (ability_name, targets) = game.ai_select_ability_and_targets(
            active_combatant_id,
            ally_battle_group,
            enemy_battle_group,
        )?;
        // get result of ability and add to list of results for this turn
        let battle = game.battles.get(&battle_id).ok_or_else(|| AppError {
            error_type: common::errors::AppErrorTypes::ServerError,
            message: error_messages::BATTLE_NOT_FOUND.to_string(),
        })?;
        let mut action_results = game.get_ability_results(
            active_combatant_id,
            &ability_name,
            &targets,
            Some(&battle.clone()),
            ally_ids,
        )?;
        // process result
        apply_action_results(game, &action_results, Some(battle.id))?;
        let party_defeated = game.all_combatants_in_group_are_dead(enemy_ids)?;
        // println!("party defeated by ai ability: {party_defeated}");

        active_combatant_turn_action_results.append(&mut action_results);
        let ability_attributes = ability_name.get_attributes();
        let combat_action_properties = ability_attributes.combat_action_properties;

        let battle = game.battles.get(&battle_id).ok_or_else(|| AppError {
            error_type: common::errors::AppErrorTypes::Generic,
            message: error_messages::BATTLE_NOT_FOUND.to_string(),
        })?;
        let turn_trackers = &battle.combatant_turn_trackers;
        let first_tracker = turn_trackers.first().ok_or_else(|| AppError {
            error_type: common::errors::AppErrorTypes::Generic,
            message: error_messages::TURN_TRACKERS_EMPTY.to_string(),
        })?;

        println!(
            "ai id {} took turn action {} with movement {}",
            active_combatant_id,
            combat_action_properties.requires_combat_turn,
            first_tracker.movement
        );

        if combat_action_properties.requires_combat_turn {
            ai_turn_results.push(CombatTurnResult {
                combatant_id: active_combatant_id,
                action_results: active_combatant_turn_action_results.clone(),
            });
            active_combatant_turn_action_results = vec![];

            let new_active_combatant_turn_tracker = game.end_active_combatant_turn(battle_id)?;
            active_combatant_id = new_active_combatant_turn_tracker.entity_id.clone();
            println!(
                "active combatant id:{:#?} turn tracker: {:#?}",
                active_combatant_id, new_active_combatant_turn_tracker
            );
            (
                active_combatant_entity_properties,
                active_combatant_properties,
            ) = game.get_combatant_by_id(&active_combatant_id)?;
            active_combatant_id = active_combatant_entity_properties.id;
            active_combatant_is_ai_controlled =
                active_combatant_properties.controlled_by == CombatantControlledBy::AI;
        }

        if party_defeated {
            return Ok(ai_turn_results);
        }
    }

    Ok(ai_turn_results)
}
