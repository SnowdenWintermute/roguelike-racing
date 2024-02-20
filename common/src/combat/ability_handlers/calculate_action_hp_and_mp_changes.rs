use crate::combat::battle::Battle;
use crate::combat::combat_actions::CombatAction;
use crate::combat::combat_actions::CombatActionTarget;
use crate::combat::ActionResult;
use crate::errors::AppError;
use crate::game::RoguelikeRacerGame;

impl RoguelikeRacerGame {
    pub fn calculate_action_hp_and_mp_changes(
        &self,
        combat_action: CombatAction,
        action_user_id: u32,
        targets: &CombatActionTarget,
        battle_option: Option<&Battle>,
        ally_ids: Vec<u32>,
        ability_level_and_base_value_scaling_factor_option: Option<(u8, f32)>,
    ) -> Result<ActionResult, AppError> {
        let mut action_result =
            ActionResult::new(action_user_id, combat_action.clone(), targets.clone());
        action_result.ends_turn = combat_action
            .get_properties_if_owned(&self, action_user_id)?
            .requires_combat_turn;

        let action_result = self.calculate_combat_action_mp_changes(
            &action_result,
            action_user_id,
            targets,
            battle_option,
            &combat_action,
        )?;

        let action_result = self.calculate_combat_action_hp_changes(
            &action_result,
            action_user_id,
            targets,
            battle_option,
            ally_ids,
            &combat_action,
            ability_level_and_base_value_scaling_factor_option,
        )?;

        Ok(action_result)
    }
}
