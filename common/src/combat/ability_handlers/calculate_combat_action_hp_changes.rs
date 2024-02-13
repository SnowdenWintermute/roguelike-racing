use crate::app_consts::error_messages;
use crate::combat::ability_handlers::add_weapon_damage_to_hp_change_range::add_weapon_damage_to_combat_action_hp_change;
use crate::combat::ability_handlers::split_combat_action_hp_change_by_number_of_targets::split_combat_action_hp_change_by_number_of_targets;
use crate::combat::battle::Battle;
use crate::combat::combat_actions::filter_possible_target_ids_by_prohibited_combatant_states::filter_possible_target_ids_by_prohibited_combatant_states;
use crate::combat::combat_actions::CombatAction;
use crate::combat::combat_actions::CombatActionTarget;
use crate::combat::combat_actions::TargetingScheme;
use crate::combat::hp_change_source_types::HpChangeSourceCategories;
use crate::combat::ActionResult;
use crate::errors::AppError;
use crate::game::RoguelikeRacerGame;
use crate::primatives::Range;
use rand::Rng;
use std::collections::HashMap;

impl RoguelikeRacerGame {
    pub fn calculate_combat_action_hp_changes(
        &self,
        action_result: &ActionResult,
        user_id: u32,
        targets: &CombatActionTarget,
        battle_option: Option<&Battle>,
        ally_ids: Vec<u32>,
        combat_action: &CombatAction,
        ability_level_and_base_value_scaling_factor_option: Option<(u8, f32)>, // ability_level_and_base_value_scaling_factor_option: Option<(u8, f32)>
    ) -> Result<ActionResult, AppError> {
        let mut action_result = action_result.clone();

        let (ally_ids, opponent_ids_option) = if let Some(battle) = battle_option {
            battle.get_ally_ids_and_opponent_ids_option(user_id)?
        } else {
            (ally_ids, None)
        };

        let combat_action_properties = combat_action.get_properties_if_owned(self, user_id)?;

        let (filtered_ally_ids, filtered_opponent_ids_option) =
            filter_possible_target_ids_by_prohibited_combatant_states(
                self,
                &combat_action_properties.prohibited_target_combatant_states,
                ally_ids.clone(),
                opponent_ids_option.clone(),
            )?;

        let target_entity_ids = targets.get_targets_if_scheme_valid(
            filtered_ally_ids,
            filtered_opponent_ids_option,
            vec![TargetingScheme::All],
        )?;

        let (_, user_combatant_properties) = self.get_combatant_by_id(&user_id)?;
        let user_combat_attributes = user_combatant_properties.get_total_attributes();

        // get hp change properties
        let hp_change_properties =
            combat_action_properties
                .hp_change_properties
                .ok_or_else(|| AppError {
                    error_type: crate::errors::AppErrorTypes::Generic,
                    message: error_messages::MISSING_ACTION_HP_CHANGE_PROPERTIES.to_string(),
                })?;
        // get base range
        let Range { min, max } = hp_change_properties.base_values;
        let mut min = min as f32;
        let mut max = max as f32;
        // add to base values if level greater than 1
        if let Some((level, level_scaling_factor)) =
            ability_level_and_base_value_scaling_factor_option
        {
            println!("level: {level} scaling: {:?}", level_scaling_factor);
            println!("min: {min} max: {max}");
            min = min * level as f32 * level_scaling_factor;
            max = max * level as f32 * level_scaling_factor;
            println!("min: {min} max: {max}");
        }
        // add scaling attribute to range
        if let Some((additive_attribute, scaling_factor)) =
            hp_change_properties.additive_attribute_and_scaling_factor
        {
            let attribute_value = user_combat_attributes
                .get(&additive_attribute)
                .unwrap_or_else(|| &0);
            let scaled_attribute_value = attribute_value * scaling_factor as u16;
            min += scaled_attribute_value as f32;
            max += scaled_attribute_value as f32;
        };
        // if weapon damage, determine main/off hand and add appropriate damage to range
        if let Some(weapon_slots) = &hp_change_properties.add_weapon_damage_from {
            let (weapon_min, weapon_max) = add_weapon_damage_to_combat_action_hp_change(
                &weapon_slots,
                &user_combatant_properties,
                &min,
                &max,
            )?;
            min += weapon_min;
            max += weapon_max;
        }
        // roll the hp change
        let mut rng = rand::thread_rng();
        let rolled = rng.gen_range(min..=max);

        println!("num targets: {}", target_entity_ids.len());

        // calculate damage split over multiple targets
        let split = split_combat_action_hp_change_by_number_of_targets(
            rolled,
            target_entity_ids.len() as f32,
        );

        action_result.hp_changes_by_entity_id = Some(HashMap::new());

        match hp_change_properties.source_properties.category {
            // if is healing, roll crit as whole then split by targets
            HpChangeSourceCategories::Healing => self
                .calculate_healing_hp_change_and_add_to_action_result(
                    &mut action_result,
                    &user_combat_attributes,
                    target_entity_ids,
                    split,
                )?,
            HpChangeSourceCategories::PhysicalDamage => self
                .calculate_physical_damage_hp_change_and_add_to_action_result(
                    &mut action_result,
                    &user_combat_attributes,
                    target_entity_ids,
                    split,
                    &hp_change_properties,
                )?,
            HpChangeSourceCategories::MagicalDamage(evadable) => self
                .calculate_magical_damage_hp_change_and_add_to_action_result(
                    &mut action_result,
                    &user_combat_attributes,
                    target_entity_ids,
                    split,
                    &hp_change_properties,
                    evadable.0,
                )?,
            HpChangeSourceCategories::Direct => todo!(),
        }
        println!("hp changes: {:#?}", &action_result.hp_changes_by_entity_id);
        Ok(action_result)
    }
}