use crate::app_consts::error_messages;
use crate::combat::battle::Battle;
use crate::combat::combat_actions::CombatActionTarget;
use crate::combat::combat_actions::TargetingScheme;
use crate::combat::ActionResult;
use crate::errors::AppError;
use crate::game::RoguelikeRacerGame;

impl RoguelikeRacerGame {
    pub fn fire_handler(
        &self,
        ability_user_id: u32,
        ability_target: &CombatActionTarget,
        battle_option: Option<&Battle>,
    ) -> Result<Vec<ActionResult>, AppError> {
        let mut action_results = vec![];
        let battle = battle_option.ok_or_else(|| AppError {
            error_type: crate::errors::AppErrorTypes::Generic,
            message: error_messages::INVALID_ABILITY_CONTEXT.to_string(),
        })?;

        let (ally_ids, opponent_ids_option) =
            battle.get_ally_ids_and_opponent_ids_option(ability_user_id)?;

        let target_entity_ids = ability_target.get_targets_if_scheme_valid(
            ally_ids,
            opponent_ids_option,
            ability_user_id,
            vec![TargetingScheme::All],
        )?;

        // get base damage from spell level and int
        // split damage between all targets
        // add bonus if multiple targeted
        // roll if crit and multiply
        // check if spell is evaded by comparing focus to obscurity
        // multiply damage by weakness/affinity traits
        // calculate resiliance % reduction/increase(if healing)

        // let (_, target_combatant_properties) =
        //     self.get_combatant_in_battle_by_id(&battle, target_entity_id)?;
        // let target_combatant_properties = target_combatant_properties.clone();
        // let (_, user_combatant_properties) =
        //     self.get_combatant_in_battle_by_id(&battle, &ability_user_id)?;
        // let user_total_attributes = user_combatant_properties.get_total_attributes();
        // let target_total_attributes = target_combatant_properties.get_total_attributes();

        Ok(action_results)
    }
}
