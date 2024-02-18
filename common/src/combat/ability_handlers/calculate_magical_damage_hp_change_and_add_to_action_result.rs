use super::apply_affinity_to_hp_change::apply_affinity_to_hp_change;
use super::apply_crit_multiplier_to_hp_change::apply_crit_multiplier_to_hp_change;
use super::roll_crit::roll_crit;
use crate::app_consts::FOCUS_TO_CRIT_CHANCE_RATIO;
use crate::app_consts::RESILIENCE_TO_PERCENT_MAGICAL_DAMAGE_REDUCTION_RATIO;
use crate::combat::combat_actions::CombatActionHpChangeProperties;
use crate::combat::ActionResult;
use crate::combatants::combat_attributes::CombatAttributes;
use crate::errors::AppError;
use crate::game::RoguelikeRacerGame;
use std::collections::HashMap;
use std::collections::HashSet;

impl RoguelikeRacerGame {
    pub fn calculate_magical_damage_hp_change_and_add_to_action_result(
        &self,
        action_result: &mut ActionResult,
        user_combat_attributes: &HashMap<CombatAttributes, u16>,
        target_entity_ids: Vec<u32>,
        rolled_hp_change_split_by_num_targets: f32,
        hp_change_properties: &CombatActionHpChangeProperties,
        evadable: bool,
    ) -> Result<(), AppError> {
        for target_id in target_entity_ids {
            let mut hp_change = rolled_hp_change_split_by_num_targets;
            // println!("hp_change initial: {}", hp_change);
            //  - if evadable roll accuracy vs evasion and return evaded
            if evadable {
                let user_accuracy = *user_combat_attributes
                    .get(&CombatAttributes::Accuracy)
                    .unwrap_or_else(|| &0) as f32
                    * (hp_change_properties.accuracy_percent_modifier as f32 / 100.0);
                let evaded = self.roll_evaded(user_accuracy as u16, target_id)?;
                if evaded {
                    action_result
                        .misses_by_entity_id
                        .get_or_insert(HashSet::new())
                        .insert(target_id);
                    continue;
                }
            }
            //  - roll crit chance
            let user_focus = user_combat_attributes
                .get(&CombatAttributes::Focus)
                .unwrap_or_else(|| &0);
            let crit_chance = *user_focus as f32 * FOCUS_TO_CRIT_CHANCE_RATIO;
            let is_crit = roll_crit(crit_chance as u16);
            //  - roll crit damage using provided multiplier stat if any
            if is_crit {
                hp_change = apply_crit_multiplier_to_hp_change(
                    &hp_change_properties,
                    &user_combat_attributes,
                    hp_change,
                );
                if let Some(crits_by_entity_id) = &mut action_result.crits_by_entity_id {
                    crits_by_entity_id.insert(target_id);
                } else {
                    action_result.crits_by_entity_id = Some(HashSet::from([target_id]));
                };
                // println!("hp_change after crit: {}", hp_change);
            }
            //  - reduce or increase damage by elemental affinity if damage type is elemental
            //     - if magical, affinity base
            let (_, target_combatant_properties) = self.get_combatant_by_id(&target_id)?;
            if let Some(element) = &hp_change_properties.source_properties.element {
                let target_affinites = target_combatant_properties.get_total_elemental_affinites();
                let target_affinity = target_affinites.get(element).unwrap_or_else(|| &0);
                let after_affinity =
                    apply_affinity_to_hp_change(*target_affinity as i16, hp_change);
                hp_change = after_affinity;
                // println!("hp_change after affinity bonus: {}", hp_change);
            }
            // apply 50% of the physical damage type if any
            if let Some(damage_type) = &hp_change_properties.source_properties.sub_category {
                let target_affinities =
                    target_combatant_properties.get_total_physical_damage_type_affinites();
                let target_affinity = target_affinities.get(&damage_type).unwrap_or_else(|| &0);
                let halved_affinity = *target_affinity as f32 / 2.0;
                let after_affinity = apply_affinity_to_hp_change(halved_affinity as i16, hp_change);
                hp_change = after_affinity;
            }
            //  reduce damage via resilience if not getting healed by affinity
            if hp_change > 0.0 {
                let target_combat_attributes = target_combatant_properties.get_total_attributes();
                let target_resilience = target_combat_attributes
                    .get(&CombatAttributes::Resilience)
                    .unwrap_or_else(|| &0);
                let penetrated_resilience =
                    std::cmp::max(0, target_resilience.saturating_sub(*user_focus));

                let damage_reduction_percentage = std::cmp::min(
                    (penetrated_resilience as f32
                        * RESILIENCE_TO_PERCENT_MAGICAL_DAMAGE_REDUCTION_RATIO)
                        as u16,
                    100,
                );
                // println!("resilience DR percentage: {}", damage_reduction_percentage);
                let damage_reduction_multiplier = 1.0 - damage_reduction_percentage as f32 / 100.0;
                // println!("DR multiplier: {}", damage_reduction_multiplier);
                hp_change *= damage_reduction_multiplier;
                // println!("hp_change after resilience: {}", hp_change);
            }
            //  - apply any base final multiplier
            hp_change *= hp_change_properties.final_damage_percent_multiplier as f32 / 100.0;
            // println!("hp_change after final multiplier: {}", hp_change);
            // as damage
            hp_change *= -1.0;
            action_result
                .hp_changes_by_entity_id
                .get_or_insert(HashMap::new())
                .insert(target_id, hp_change as i16);
        }
        Ok(())
    }
}
