use super::apply_crit_multiplier_to_hp_change::apply_crit_multiplier_to_hp_change;
use super::apply_elemental_affinity_to_hp_change::apply_elemental_affinity_to_hp_change;
use super::roll_crit::roll_crit;
use crate::app_consts::BASE_CRIT_CHANCE;
use crate::app_consts::BASE_CRIT_MULTIPLIER;
use crate::app_consts::FOCUS_TO_CRIT_CHANCE_RATIO;
use crate::app_consts::VIT_TO_PERCENT_PHYSICAL_DAMAGE_REDUCTION_RATION;
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
        let mut hp_change = rolled_hp_change_split_by_num_targets;
        for target_id in target_entity_ids {
            //  - if evadable roll accuracy vs evasion and return evaded
            if evadable {
                let user_accuracy = user_combat_attributes
                    .get(&CombatAttributes::Accuracy)
                    .unwrap_or_else(|| &0);
                let evaded = self.roll_evaded(*user_accuracy, target_id)?;
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
            }
            //  reduce damage via resilience
            //  - reduce or increase damage by elemental affinity if damage type is elemental
            //     - if magical, affinity base
            //     - if physical, affinity effect is halved
            //  - apply any base final multiplier
            let user_dex = user_combat_attributes
                .get(&CombatAttributes::Dexterity)
                .unwrap_or_else(|| &0);
            let (_, target_combatant_properties) = self.get_combatant_by_id(&target_id)?;
            let target_combat_attributes = target_combatant_properties.get_total_attributes();
            let target_agi = target_combat_attributes
                .get(&CombatAttributes::Agility)
                .unwrap_or_else(|| &0);
            let crit_chance_after_reduction = BASE_CRIT_CHANCE + user_dex - target_agi;
            let is_crit = roll_crit(crit_chance_after_reduction);
            if is_crit {
                //  get provided multiplier stat if any
                let crit_multiplier_from_attribute = if let Some(crit_multiplier_attribute) =
                    hp_change_properties.crit_multiplier_attribute
                {
                    user_combat_attributes
                        .get(&crit_multiplier_attribute)
                        .unwrap_or_else(|| &0)
                } else {
                    &0
                };
                let crit_multiplier =
                    BASE_CRIT_MULTIPLIER + (*crit_multiplier_from_attribute as f32 / 100.0);
                hp_change *= crit_multiplier;
            }
            //  - reduce damage via target armor vs target armor pen
            let target_ac = target_combat_attributes
                .get(&CombatAttributes::ArmorClass)
                .unwrap_or_else(|| &0);
            let user_armor_pen = user_combat_attributes
                .get(&CombatAttributes::ArmorPenetration)
                .unwrap_or_else(|| &0);
            let final_ac = target_ac.saturating_sub(*user_armor_pen) as u32;
            let damage_after_ac = hp_change.powf(2.0) / (final_ac as f32 + hp_change);
            //  reduce damage via vit
            let target_vit = target_combat_attributes
                .get(&CombatAttributes::Vitality)
                .unwrap_or_else(|| &0);

            let damage_reduction_percentage = std::cmp::min(
                (*target_vit as f32 * VIT_TO_PERCENT_PHYSICAL_DAMAGE_REDUCTION_RATION) as u16,
                100,
            );
            let damage_reduction_multiplier = 1.0 - damage_reduction_percentage as f32 / 100.0;
            hp_change = damage_after_ac * damage_reduction_multiplier;

            //  - reduce or increase damage by elemental affinity if damage type is elemental
            //     - if physical, affinity effect is halved
            if let Some(element) = &hp_change_properties.source_properties.element {
                let target_affinites = target_combatant_properties.get_total_elemental_affinites();
                let target_affinity = target_affinites.get(element).unwrap_or_else(|| &0);
                let halved_affinity = *target_affinity as f32 / 2.0;
                let after_affinity =
                    apply_elemental_affinity_to_hp_change(halved_affinity as i16, hp_change);
                hp_change = after_affinity;
            }
            //  - apply any base final multiplier
            hp_change *= hp_change_properties.base_final_percent_multiplier as f32 / 100.0;
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
