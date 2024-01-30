use crate::store::game_store::get_cloned_current_battle_option;
use crate::store::game_store::GameStore;
use common::app_consts::error_messages;
use common::combat::combat_actions::filter_possible_target_ids_by_prohibited_combatant_states::filter_possible_target_ids_by_prohibited_combatant_states;
use common::combat::combat_actions::CombatActionProperties;
use common::combat::combat_actions::CombatActionTarget;
use common::errors::AppError;
use common::game::getters::get_ally_ids_and_opponent_ids_option;
use common::game::getters::get_mut_party;
use common::primatives::NextOrPrevious;

pub fn handle_cycle_combat_action_targets(
    game_store: &mut GameStore,
    combat_action_properties: CombatActionProperties,
    direction: &NextOrPrevious,
) -> Result<CombatActionTarget, AppError> {
    let battle_option = get_cloned_current_battle_option(&game_store);
    let game = game_store.game.as_mut().ok_or_else(|| AppError {
        error_type: common::errors::AppErrorTypes::ClientError,
        message: error_messages::MISSING_GAME_REFERENCE.to_string(),
    })?;
    let party_id = game_store.current_party_id.ok_or_else(|| AppError {
        error_type: common::errors::AppErrorTypes::ClientError,
        message: error_messages::MISSING_PARTY_REFERENCE.to_string(),
    })?;
    let party = get_mut_party(game, party_id)?;
    let cloned_character_positions = party.character_positions.clone();
    let focused_character = party
        .characters
        .get(&game_store.focused_character_id)
        .ok_or_else(|| AppError {
            error_type: common::errors::AppErrorTypes::ClientError,
            message: error_messages::CHARACTER_NOT_FOUND.to_string(),
        })?;
    let focused_character_id = focused_character.entity_properties.id;
    let prohibited_target_combatant_states = combat_action_properties
        .prohibited_target_combatant_states
        .clone();

    let current_targets = focused_character
        .combatant_properties
        .combat_action_targets
        .as_ref()
        .ok_or_else(|| AppError {
            error_type: common::errors::AppErrorTypes::ClientError,
            message: error_messages::TRIED_TO_CYCLE_TARGETS_WHEN_NO_TARGETS.to_string(),
        })?;
    let current_targets = current_targets.clone();

    let (ally_ids, opponent_ids_option) = get_ally_ids_and_opponent_ids_option(
        &cloned_character_positions,
        battle_option.as_ref(),
        focused_character_id,
    )?;

    let (ally_ids, opponent_ids_option) =
        filter_possible_target_ids_by_prohibited_combatant_states(
            game,
            &prohibited_target_combatant_states,
            ally_ids,
            opponent_ids_option,
        )?;

    let new_targets = combat_action_properties.get_next_or_previous_targets(
        &current_targets,
        direction,
        &focused_character_id,
        &ally_ids,
        &opponent_ids_option,
    )?;

    let party = get_mut_party(game, party_id)?;
    let focused_character = party
        .characters
        .get_mut(&game_store.focused_character_id)
        .ok_or_else(|| AppError {
            error_type: common::errors::AppErrorTypes::ClientError,
            message: error_messages::CHARACTER_NOT_FOUND.to_string(),
        })?;
    let new_preferences = focused_character
        .combatant_properties
        .combat_action_target_preferences
        .get_updated_preferences(
            &combat_action_properties,
            &new_targets,
            ally_ids,
            opponent_ids_option,
        );
    focused_character
        .combatant_properties
        .combat_action_target_preferences = new_preferences;

    Ok(new_targets)
}
