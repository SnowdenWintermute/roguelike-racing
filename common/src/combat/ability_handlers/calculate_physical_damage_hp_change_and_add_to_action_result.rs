use super::apply_crit_multiplier_to_hp_change::apply_crit_multiplier_to_hp_change;
use super::apply_elemental_affinity_to_hp_change::apply_elemental_affinity_to_hp_change;
use super::roll_crit::roll_crit;
use crate::app_consts::BASE_CRIT_CHANCE;
use crate::app_consts::BASE_CRIT_MULTIPLIER;
use crate::app_consts::VIT_TO_PERCENT_PHYSICAL_DAMAGE_REDUCTION_RATION;
use crate::combat::combat_actions::CombatActionHpChangeProperties;
use crate::combat::ActionResult;
use crate::combatants::combat_attributes::CombatAttributes;
use crate::errors::AppError;
use crate::game::RoguelikeRacerGame;
use std::collections::HashMap;

impl RoguelikeRacerGame {
    pub fn calculate_physical_damage_hp_change_and_add_to_action_result(
        &self,
        action_result: &mut ActionResult,
        user_combat_attributes: &HashMap<CombatAttributes, u16>,
        target_entity_ids: Vec<u32>,
        rolled_hp_change_split_by_num_targets: f32,
        hp_change_properties: &CombatActionHpChangeProperties,
    ) -> Result<(), AppError> {
        let mut hp_change = rolled_hp_change_split_by_num_targets;
        //  - get crit chance vs crit chance reduction
        //  - roll crit chance vs %chance reduction from AGI if physical
        for target_id in target_entity_ids {
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
                hp_change = apply_crit_multiplier_to_hp_change(
                    &hp_change_properties,
                    &user_combat_attributes,
                    hp_change,
                );
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
