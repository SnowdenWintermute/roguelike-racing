use common::app_consts::error_messages;
use common::combat::CombatTurnResult;
use common::combatants::CombatantControlledBy;
use common::errors::AppError;
use common::game::RoguelikeRacerGame;

use crate::websocket_server::game_server::player_input_handlers::in_game::apply_action_results::apply_action_results;

pub fn take_ai_controlled_turns(
    game: &mut RoguelikeRacerGame,
    battle_id: u32,
) -> Result<Vec<CombatTurnResult>, AppError> {
    let mut ai_turn_results = vec![];
    let active_combatant_turn_tracker = game.end_active_combatant_turn(battle_id)?;
    let active_combatant_turn_tracker = active_combatant_turn_tracker.clone();

    let (mut active_combatant_entity_properties, mut active_combatant_properties) =
        game.get_combatant_by_id(&active_combatant_turn_tracker.entity_id)?;
    let mut active_combatant_entity_id = active_combatant_entity_properties.id;
    let mut active_combatant_is_ai_controlled =
        active_combatant_properties.controlled_by == CombatantControlledBy::AI;
    let mut active_combatant_turn_action_results = vec![];

    while active_combatant_is_ai_controlled {
        let battle = game.battles.get(&battle_id).ok_or_else(|| AppError {
            error_type: common::errors::AppErrorTypes::ServerError,
            message: error_messages::BATTLE_NOT_FOUND.to_string(),
        })?;
        let (ally_battle_group, enemy_battle_group) =
            battle.get_ally_and_enemy_battle_groups(&active_combatant_entity_id)?;
        let (ability_name, targets) = game.ai_select_ability_and_targets(
            active_combatant_entity_id,
            ally_battle_group,
            enemy_battle_group,
        )?;
        // get result of ability and add to list of results for this turn
        let battle = game.battles.get(&battle_id).ok_or_else(|| AppError {
            error_type: common::errors::AppErrorTypes::ServerError,
            message: error_messages::BATTLE_NOT_FOUND.to_string(),
        })?;
        let mut action_results = game.get_ability_results(
            active_combatant_entity_id,
            &ability_name,
            &targets,
            Some(&battle.clone()),
        )?;
        // process result
        apply_action_results(game, &action_results)?;
        // @TODO - check if one of the battle_groups is completely defeated and end the
        // battle if so

        active_combatant_turn_action_results.append(&mut action_results);

        if ability_name.get_attributes().requires_combat_turn {
            ai_turn_results.push(CombatTurnResult {
                combatant_id: active_combatant_entity_id,
                action_results: active_combatant_turn_action_results.clone(),
            });
            // end the active_combatant's turn and check if new active combatant
            // is a player or AI
            let new_active_combatant_turn_tracker = game.end_active_combatant_turn(battle_id)?;
            active_combatant_entity_id = new_active_combatant_turn_tracker.entity_id.clone();
            (
                active_combatant_entity_properties,
                active_combatant_properties,
            ) = game.get_combatant_by_id(&active_combatant_entity_id)?;
            active_combatant_entity_id = active_combatant_entity_properties.id;
            active_combatant_is_ai_controlled =
                active_combatant_properties.controlled_by == CombatantControlledBy::AI;
        } else {
            continue;
        }
    }

    Ok(ai_turn_results)
}
