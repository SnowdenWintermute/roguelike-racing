use super::roll_crit::roll_crit;
use crate::app_consts::BASE_CRIT_CHANCE;
use crate::app_consts::BASE_CRIT_MULTIPLIER;
use crate::combat::ActionResult;
use crate::combatants::combat_attributes::CombatAttributes;
use crate::errors::AppError;
use crate::game::RoguelikeRacerGame;
use std::collections::HashMap;
use std::collections::HashSet;

impl RoguelikeRacerGame {
    pub fn calculate_healing_hp_change_and_add_to_action_result(
        &self,
        action_result: &mut ActionResult,
        user_combat_attributes: &HashMap<CombatAttributes, u16>,
        target_entity_ids: Vec<u32>,
        rolled_hp_change_split_by_num_targets: f32,
    ) -> Result<(), AppError> {
        println!("calculating healing");
        let user_focus_attribute = user_combat_attributes
            .get(&CombatAttributes::Focus)
            .unwrap_or_else(|| &0);
        //  - roll crit chance
        let is_crit = roll_crit(BASE_CRIT_CHANCE + user_focus_attribute);
        //  - add crit multiplier
        let mut hp_change_initial = rolled_hp_change_split_by_num_targets;
        if is_crit {
            let crit_multiplier = *user_focus_attribute as f32 / 100.0 + BASE_CRIT_MULTIPLIER;
            hp_change_initial *= crit_multiplier;
        }

        for target_id in target_entity_ids {
            let hp_change = hp_change_initial;
            let target_hp_change =
                self.get_healing_hp_change_on_target_combatant(&target_id, hp_change)?;
            action_result
                .hp_changes_by_entity_id
                .get_or_insert(HashMap::new())
                .insert(target_id, target_hp_change as i16);
            if is_crit {
                action_result
                    .crits_by_entity_id
                    .get_or_insert(HashSet::new())
                    .insert(target_id);
            }
        }
        Ok(())
    }
}
