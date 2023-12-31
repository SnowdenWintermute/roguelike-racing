use crate::combatants::combat_attributes::CombatAttributes;
use std::cmp;
use std::collections::HashMap;

pub fn physical_damage_after_defense(
    user_total_attributes: &HashMap<CombatAttributes, u16>,
    target_total_attributes: &HashMap<CombatAttributes, u16>,
    rolled_damage: u32,
) -> u16 {
    let target_ac = target_total_attributes
        .get(&CombatAttributes::ArmorClass)
        .unwrap_or_else(|| &0);
    let user_armor_pen = user_total_attributes
        .get(&CombatAttributes::ArmorPenetration)
        .unwrap_or_else(|| &0);
    let penetrated_ac = target_ac.saturating_sub(*user_armor_pen) as u32;
    let damage_after_ac = (5 * rolled_damage.pow(2)) / penetrated_ac + 5 * rolled_damage;

    let physical_damage_reduction = cmp::max(
        user_total_attributes
            .get(&CombatAttributes::Vitality)
            .unwrap_or_else(|| &0),
        &100,
    );

    let max_u16_value: u32 = u16::MAX as u32;
    let damage_after_damage_reduction =
        damage_after_ac - damage_after_ac * (*physical_damage_reduction as u32 / 100);
    let final_damage: u16 = damage_after_damage_reduction.clamp(0, max_u16_value) as u16;
    final_damage
}
