use crate::combat::ability_handlers::add_weapon_damage_to_hp_change_range::add_weapon_damage_to_combat_action_hp_change;
use crate::combat::combat_actions::CombatActionHpChangeProperties;
use crate::combatants::CombatantProperties;
use crate::errors::AppError;
use crate::game::RoguelikeRacerGame;
use crate::primatives::Range;

impl RoguelikeRacerGame {
    pub fn calculate_combat_action_hp_change_range(
        &self,
        user_combatant_properties: &CombatantProperties,
        hp_change_properties: &CombatActionHpChangeProperties,
        ability_level_and_base_value_scaling_factor_option: &Option<(u8, f32)>, // ability_level_and_base_value_scaling_factor_option: Option<(u8, f32)>
    ) -> Result<(f32, f32), AppError> {
        let user_combat_attributes = user_combatant_properties.get_total_attributes();

        // get base range
        let Range { min, max } = hp_change_properties.base_values;
        let mut min = min as f32;
        let mut max = max as f32;

        // add to base values if level greater than 1
        if let Some((level, level_scaling_factor)) =
            ability_level_and_base_value_scaling_factor_option
        {
            min = min * *level as f32 * level_scaling_factor;
            max = max * *level as f32 * level_scaling_factor;
        }
        // add scaling attribute to range
        if let Some((additive_attribute, percent_scaling_factor)) =
            hp_change_properties.additive_attribute_and_percent_scaling_factor
        {
            let attribute_value = user_combat_attributes
                .get(&additive_attribute)
                .unwrap_or_else(|| &0);
            let scaled_attribute_value =
                *attribute_value as f32 * (percent_scaling_factor as f32 / 100.0);
            min += scaled_attribute_value;
            max += scaled_attribute_value;
        };
        println!("damage after attribute min: {min} max: {max}");
        // if weapon damage, determine main/off hand and add appropriate damage to range
        if let Some(weapon_slots) = &hp_change_properties.add_weapon_damage_from {
            (min, max) = add_weapon_damage_to_combat_action_hp_change(
                &weapon_slots,
                &user_combatant_properties,
                &min,
                &max,
            )?;
        }
        Ok((min, max))
    }
}
