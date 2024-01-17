use crate::app_consts::error_messages;
use crate::combat::battle::Battle;
use crate::combatants::abilities::get_combatant_ability_attributes::AbilityUsableContext;
use crate::combatants::abilities::get_combatant_ability_attributes::CombatantAbilityAttributes;
use crate::combatants::abilities::AbilityTarget;
use crate::combatants::abilities::CombatantAbilityNames;
use crate::errors::AppError;
use crate::errors::AppErrorTypes;
use crate::game::RoguelikeRacerGame;

pub fn validate_character_ability_use(
    ability_name: &CombatantAbilityNames,
    ability_attributes: &CombatantAbilityAttributes,
    battle_option: Option<&Battle>,
    ally_ids: &Vec<u32>,
    targets: &AbilityTarget,
    character_id: u32,
    game: &RoguelikeRacerGame,
) -> Result<(), AppError> {
    let (ally_ids, opponent_ids_option) = if let Some(battle) = battle_option {
        // check if character is first in turn order
        if !battle.combatant_is_first_in_turn_order(character_id) {
            return Err(AppError {
                error_type: AppErrorTypes::InvalidInput,
                message: error_messages::NOT_THIS_COMBATANTS_TURN.to_string(),
            });
        }
        // check if ability is usable in combat
        if ability_attributes.usability_context == AbilityUsableContext::OutOfCombat {
            return Err(AppError {
                error_type: AppErrorTypes::InvalidInput,
                message: error_messages::INVALID_ABILITY_CONTEXT.to_string(),
            });
        };
        battle.get_ally_ids_and_opponent_ids_option(character_id)?
    } else {
        (ally_ids.clone(), None)
    };

    // check if targets are valid
    let targets_are_valid = ability_name.targets_are_valid(
        character_id,
        &targets,
        &ally_ids,
        &opponent_ids_option,
        game,
    );

    if !targets_are_valid {
        return Err(AppError {
            error_type: AppErrorTypes::InvalidInput,
            message: error_messages::INVALID_TARGETS_SELECTED.to_string(),
        });
    }

    Ok(())
}
