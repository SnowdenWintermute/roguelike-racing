use crate::app_consts::error_messages;
use crate::combat::ability_handlers::add_weapon_damage_to_hp_change_range::add_weapon_damage_to_combat_action_hp_change;
use crate::combat::ability_handlers::split_combat_action_hp_change_by_number_of_targets::split_combat_action_hp_change_by_number_of_targets;
use crate::combat::battle::Battle;
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
        combat_action: &CombatAction,
    ) -> Result<ActionResult, AppError> {
        let mut action_result = action_result.clone();
        let battle = battle_option.ok_or_else(|| AppError {
            error_type: crate::errors::AppErrorTypes::Generic,
            message: error_messages::INVALID_ABILITY_CONTEXT.to_string(),
        })?;

        let (ally_ids, opponent_ids_option) =
            battle.get_ally_ids_and_opponent_ids_option(user_id)?;

        // MAKE THIS FILTER PROHIBITED STATES
        let target_entity_ids = targets.get_targets_if_scheme_valid(
            ally_ids,
            opponent_ids_option,
            vec![TargetingScheme::All],
        )?;

        let (_, user_combatant_properties) = self.get_combatant_by_id(&user_id)?;
        let user_combat_attributes = user_combatant_properties.get_total_attributes();
        let combat_action_properties = match &combat_action {
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
        // get base range
        let Range { min, max } = hp_change_properties.base_values;
        let mut min = min as f32;
        let mut max = max as f32;
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
        Ok(action_result)
    }
}
