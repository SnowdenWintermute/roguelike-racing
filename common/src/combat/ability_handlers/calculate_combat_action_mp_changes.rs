use crate::app_consts::error_messages;
use crate::combat::battle::Battle;
use crate::combat::combat_actions::CombatAction;
use crate::combat::combat_actions::CombatActionTarget;
use crate::combat::ActionResult;
use crate::errors::AppError;
use crate::game::RoguelikeRacerGame;
use std::collections::HashMap;

impl RoguelikeRacerGame {
    pub fn calculate_combat_action_mp_changes(
        &self,
        action_result: &ActionResult,
        user_id: u32,
        _targets: &CombatActionTarget,
        _battle_option: Option<&Battle>,
        combat_action: &CombatAction,
    ) -> Result<ActionResult, AppError> {
        let mut action_result = action_result.clone();
        let (ability_name, _) = match combat_action {
            CombatAction::AbilityUsed(ability_name) => {
                (ability_name, ability_name.get_attributes())
            }
            CombatAction::ConsumableUsed(_) => return Ok(action_result),
        };
        let (_, user_combatant_properties) = self.get_combatant_by_id(&user_id)?;
        let level_adjusted_mp_cost =
            user_combatant_properties.get_ability_cost_if_owned(&ability_name)?;

        if user_combatant_properties.mana < level_adjusted_mp_cost as u16 {
            return Err(AppError {
                error_type: crate::errors::AppErrorTypes::InvalidInput,
                message: error_messages::INSUFFICIENT_MP.to_string(),
            });
        }

        action_result
            .mp_combat_action_prices_paid_by_entity_id
            .get_or_insert(HashMap::new())
            .insert(user_id, level_adjusted_mp_cost);

        Ok(action_result)
    }
}
