use crate::app_consts::error_messages;
use crate::combat::battle::Battle;
use crate::combat::combat_actions::AbilityUsableContext;
use crate::combat::combat_actions::CombatActionProperties;
use crate::combat::combat_actions::CombatActionTarget;
use crate::errors::AppError;
use crate::errors::AppErrorTypes;

impl CombatActionProperties {
    pub fn validate_use(
        &self,
        battle_option: Option<&Battle>,
        ally_ids: &Vec<u32>,
        targets: &CombatActionTarget,
        character_id: u32,
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
            if self.usability_context == AbilityUsableContext::OutOfCombat {
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
        let targets_are_valid =
            self.targets_are_valid(character_id, &targets, &ally_ids, &opponent_ids_option);

        if !targets_are_valid {
            return Err(AppError {
                error_type: AppErrorTypes::InvalidInput,
                message: error_messages::INVALID_TARGETS_SELECTED.to_string(),
            });
        }

        Ok(())
    }
}
