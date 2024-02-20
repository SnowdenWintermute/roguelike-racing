use crate::app_consts::error_messages;
use crate::combat::ability_handlers::split_combat_action_hp_change_by_number_of_targets::split_combat_action_hp_change_by_number_of_targets;
use crate::combat::battle::Battle;
use crate::combat::combat_actions::filter_possible_target_ids_by_prohibited_combatant_states::filter_possible_target_ids_by_prohibited_combatant_states;
use crate::combat::combat_actions::CombatAction;
use crate::combat::combat_actions::CombatActionTarget;
use crate::combat::hp_change_source_types::HpChangeSourceCategories;
use crate::combat::ActionResult;
use crate::errors::AppError;
use crate::game::RoguelikeRacerGame;
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
            vec![],
        )?;

        let (_, user_combatant_properties) = self.get_combatant_by_id(&user_id)?;
        // get hp change properties and make mutable so we can change the element if appropriate
        let mut hp_change_properties = combat_action_properties
            .hp_change_properties
            .clone()
            .ok_or_else(|| AppError {
                error_type: crate::errors::AppErrorTypes::Generic,
                message: error_messages::MISSING_ACTION_HP_CHANGE_PROPERTIES.to_string(),
            })?;
        let user_combat_attributes = user_combatant_properties.get_total_attributes();

        let (min, max) = self.calculate_combat_action_hp_change_range(
            &user_combatant_properties,
            &hp_change_properties,
            &ability_level_and_base_value_scaling_factor_option,
        )?;

        // ADJUST HP CHANGE SOURCE ELEMENT FROM WEAPON ELEMENT IF APPROPRIATE, ADDING THE MOST
        // DAMAGING ONE
        let weapon_slot_to_add_element_from_option =
            hp_change_properties.add_weapon_element_from.clone();
        if let Some(weapon_slot) = &weapon_slot_to_add_element_from_option {
            self.add_element_from_weapon_to_hp_change_properties(
                &weapon_slot,
                &user_combatant_properties,
                &target_entity_ids,
                &mut hp_change_properties,
            )?;
        };
        // ADJUST HP CHANGE SOURCE PHYSICAL DAMAGE TYPE FROM WEAPON ELEMENT IF APPROPRIATE, ADDING THE MOST
        // DAMAGING ONE
        let weapon_slot_to_add_damage_type_from_option =
            hp_change_properties.add_weapon_damage_type_from.clone();
        if let Some(weapon_slot) = &weapon_slot_to_add_damage_type_from_option {
            self.add_damage_type_from_weapon_to_hp_change_properties(
                &weapon_slot,
                &user_combatant_properties,
                &target_entity_ids,
                &mut hp_change_properties,
            )?;
        };

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
            HpChangeSourceCategories::PhysicalDamage(melee_or_ranged) => self
                .calculate_physical_damage_hp_change_and_add_to_action_result(
                    &mut action_result,
                    melee_or_ranged,
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
