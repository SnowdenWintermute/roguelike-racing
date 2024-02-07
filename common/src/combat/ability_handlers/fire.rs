use super::apply_elemental_affinity_to_hp_change_range::apply_elemental_affinity_to_hp_change_range;
use super::get_ability_base_hp_change_range::get_ability_base_hp_change_range;
use super::get_crit_range::get_crit_range;
use super::roll_crit::roll_crit;
use super::split_combat_action_hp_change_by_number_of_targets::split_combat_action_hp_change_by_number_of_targets;
use crate::app_consts::error_messages;
use crate::app_consts::RESILIENCE_TO_PERCENT_EFFECT_ON_MAGICAL_DAMAGE_RATIO;
use crate::combat::battle::Battle;
use crate::combat::combat_actions::CombatAction;
use crate::combat::combat_actions::CombatActionTarget;
use crate::combat::combat_actions::TargetingScheme;
use crate::combat::ActionResult;
use crate::combatants::abilities::CombatantAbilityNames;
use crate::combatants::combat_attributes::CombatAttributes;
use crate::errors::AppError;
use crate::game::RoguelikeRacerGame;
use crate::items::equipment::unarmed::FIST;
use crate::items::equipment::EquipmentSlots;
use crate::primatives::Range;
use crate::primatives::WeaponSlot;
use rand::Rng;
use std::collections::HashMap;

impl RoguelikeRacerGame {
    pub fn calculate_combat_action_hp_changes(
        &self,
        ability_user_id: u32,
        ability_target: &CombatActionTarget,
        battle_option: Option<&Battle>,
        combat_action: CombatAction,
    ) -> Result<ActionResult, AppError> {
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

        let (_, user_combatant_properties) = self.get_combatant_by_id(&ability_user_id)?;
        let user_combat_attributes = user_combatant_properties.get_total_attributes();
        let user_focus_attribute = user_combat_attributes
            .get(&CombatAttributes::Focus)
            .unwrap_or_else(|| &0);
        let ability =
            user_combatant_properties.get_ability_if_owned(&CombatantAbilityNames::Fire)?;
        let combat_action_properties = match combat_action {
            CombatAction::AbilityUsed(ability_name) => {
                ability_name.get_attributes().combat_action_properties
            }
            CombatAction::ConsumableUsed(_) => todo!(),
        };

        // get hp change properties
        let hp_change_properties =
            combat_action_properties
                .hp_change_properties
                .ok_or_else(|| AppError {
                    error_type: crate::errors::AppErrorTypes::Generic,
                    message: error_messages::MISSING_ACTION_HP_CHANGE_PROPERTIES.to_string(),
                })?;
        // add scaling attribute to range
        let Range { mut min, mut max } = hp_change_properties.base_values;
        // if weapon damage, determine main/off hand and add appropriate damage to range
        if let Some(weapon_slots) = hp_change_properties.add_weapon_damage_from {
            for weapon_slot in weapon_slots {
                match weapon_slot {
                    WeaponSlot::MainHand => {
                        let mh_equipment_option = user_combatant_properties
                            .get_equipped_item(&EquipmentSlots::MainHand)
                            .unwrap_or(&FIST);
                    }
                    WeaponSlot::OffHand => todo!(),
                }
            }
        }
        // calculate damage split over multiple targets
        // if is healing, on each target
        //  - roll crit chance
        //  - roll crit multiplier
        //  - add Resilience multiplier
        //  - if has undead trait, convert to damage
        // on each target
        //  - if physical or magical evadable
        //    - roll accuracy vs evasion and return evaded
        //  - roll crit chance vs %chance reduction from AGI if physical
        //  - roll crit damage using provided multiplier stat if any
        //  - if damage type physical, reduce damage via target armor vs target armor pen
        //  - if damage type magical, reduce damage via resilience
        //  - reduce or increase damage by elemental affinity if damage type is elemental
        //     - if magical, affinity base
        //     - if physical, affinity effect is halved

        if let Some((scaling_attribute, scaling_attribute_factor)) =
            hp_change_properties.scaling_attribute_and_factor
        {
            //
        };

        // let intelligence = user_combat_attributes
        //     .get(&CombatAttributes::Intelligence)
        //     .unwrap_or_else(|| &0);

        // // get base damage from spell level and int
        // let (min_base_damage, max_base_damage) =
        //     get_ability_base_hp_change_range(&ability, *intelligence)?;

        // // add bonus if multiple targeted
        // let num_targets = target_entity_ids.len();
        // let (mut min_hp_change, mut max_hp_change) =
        //     split_combat_action_hp_change_by_number_of_targets(
        //         min_base_damage,
        //         max_base_damage,
        //         num_targets as u8,
        //     );
        // // roll if crit and multiply
        // let spell_crit_chance = user_focus_attribute;
        // let is_crit = roll_crit(*spell_crit_chance);
        // if is_crit {
        //     (min_hp_change, max_hp_change) =
        //         get_crit_range(*user_focus_attribute, min_hp_change, max_hp_change);
        // }

        // let mut hp_changes_by_entity_id = HashMap::new();

        // for target_id in target_entity_ids {
        //     let (_, target_combatant_properties) =
        //         self.get_combatant_in_battle_by_id(&battle, &target_id)?;
        //     let target_elemental_affinites =
        //         target_combatant_properties.get_total_elemental_affinites();
        //     let affinity = target_elemental_affinites
        //         .get(&element)
        //         .unwrap_or_else(|| &0);
        //     // multiply damage by weakness/affinity traits
        //     let (min_hp_change, max_hp_change) = apply_elemental_affinity_to_hp_change_range(
        //         *affinity,
        //         min_hp_change,
        //         max_hp_change,
        //     );

        //     // calculate resiliance % reduction/increase(if healing)
        //     let target_total_attributes = target_combatant_properties.get_total_attributes();
        //     let resilience = target_total_attributes
        //         .get(&CombatAttributes::Resilience)
        //         .unwrap_or_else(|| &0);
        //     let resilience_modifer = if min_hp_change >= 0.0 {
        //         RESILIENCE_TO_PERCENT_EFFECT_ON_MAGICAL_DAMAGE_RATIO * *resilience as f32 / 100.0
        //     } else {
        //         RESILIENCE_TO_PERCENT_EFFECT_ON_MAGICAL_DAMAGE_RATIO * *resilience as f32 / 100.0
        //             + 1.0
        //     };

        //     let min_hp_change = min_hp_change * resilience_modifer;
        //     let max_hp_change = max_hp_change * resilience_modifer;
        //     let mut rng = rand::thread_rng();
        //     let rolled_hp_change = rng.gen_range(min_hp_change..=max_hp_change);
        //     hp_changes_by_entity_id.insert(target_id, rolled_hp_change);
        // }

        // Ok(action_results)
        todo!()
    }
}
