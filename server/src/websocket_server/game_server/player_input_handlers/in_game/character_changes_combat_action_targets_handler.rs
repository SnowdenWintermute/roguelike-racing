use common::app_consts::error_messages;
use common::combat::combat_actions::filter_possible_target_ids_by_prohibited_combatant_states::filter_possible_target_ids_by_prohibited_combatant_states;
use common::combat::combat_actions::CombatActionProperties;
use common::combat::combat_actions::CombatActionTarget;
use common::combatants::CombatActionTargetPreferences;
use common::errors::AppError;
use common::errors::AppErrorTypes;
use common::game::RoguelikeRacerGame;
use std::collections::HashSet;

pub fn character_changes_combat_action_targets_handler(
    game: &mut RoguelikeRacerGame,
    new_targets: CombatActionTarget,
    combat_action_properties: CombatActionProperties,
    character_id: u32,
    battle_id_option: Option<u32>,
    character_positions: Vec<u32>,
    player_character_ids_option: Option<HashSet<u32>>,
    party_id: u32,
) -> Result<(CombatActionTarget, CombatActionTargetPreferences), AppError> {
    let (ally_ids, opponent_ids_option) = if let Some(battle_id) = battle_id_option {
        let battle = game.battles.get(&battle_id).ok_or_else(|| AppError {
            error_type: AppErrorTypes::Generic,
            message: error_messages::BATTLE_NOT_FOUND.to_string(),
        })?;

        battle.get_ally_ids_and_opponent_ids_option(character_id)?
    } else {
        (character_positions.clone(), None)
    };

    let prohibited_target_combatant_states = combat_action_properties
        .prohibited_target_combatant_states
        .clone();
    let (ally_ids, opponent_ids_option) =
        filter_possible_target_ids_by_prohibited_combatant_states(
            game,
            &prohibited_target_combatant_states,
            ally_ids,
            opponent_ids_option,
        )?;

    let new_targets = if combat_action_properties.targets_are_valid(
        character_id,
        &new_targets,
        &ally_ids,
        &opponent_ids_option,
    ) {
        new_targets
    } else {
        combat_action_properties.get_default_targets(
            character_id,
            &ally_ids,
            &opponent_ids_option,
        )?
    };

    let party = game
        .adventuring_parties
        .get(&party_id)
        .ok_or_else(|| AppError {
            error_type: AppErrorTypes::ServerError,
            message: error_messages::PARTY_NOT_FOUND.to_string(),
        })?;
    let character =
        party.get_character_if_owned(player_character_ids_option.clone(), character_id)?;

    let target_preferences = &character
        .combatant_properties
        .combat_action_target_preferences;
    let new_target_preferences = target_preferences.get_updated_preferences(
        &combat_action_properties,
        &new_targets,
        ally_ids,
        opponent_ids_option,
    );

    Ok((new_targets, new_target_preferences))
}
