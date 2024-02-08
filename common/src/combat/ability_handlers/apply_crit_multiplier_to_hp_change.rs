use crate::app_consts::BASE_CRIT_MULTIPLIER;
use crate::combat::combat_actions::CombatActionHpChangeProperties;
use crate::combatants::combat_attributes::CombatAttributes;
use std::collections::HashMap;

pub fn apply_crit_multiplier_to_hp_change(
    hp_change_properties: &CombatActionHpChangeProperties,
    user_combat_attributes: &HashMap<CombatAttributes, u16>,
    hp_change: f32,
) -> f32 {
    let crit_multiplier_from_attribute =
        if let Some(crit_multiplier_attribute) = hp_change_properties.crit_multiplier_attribute {
            user_combat_attributes
                .get(&crit_multiplier_attribute)
                .unwrap_or_else(|| &0)
        } else {
            &0
        };
    let crit_multiplier = BASE_CRIT_MULTIPLIER + (*crit_multiplier_from_attribute as f32 / 100.0);
    hp_change * crit_multiplier
}
