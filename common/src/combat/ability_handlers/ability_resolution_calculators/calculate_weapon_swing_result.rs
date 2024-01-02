use super::physical_attack_evaded::physical_attack_evaded;
use super::physical_damage_after_defense::physical_damage_after_defense;
use crate::combat::CombatAction;
use crate::combat::CombatActionResult;
use crate::combat::IdAndValue;
use crate::combatants::abilities::AbilityTarget;
use crate::combatants::abilities::CombatantAbilityNames;
use crate::combatants::combat_attributes::CombatAttributes;
use crate::combatants::get_weapon_properties_traits_and_base_bonus_damage::get_weapon_properties_traits_and_base_bonus_damage;
use crate::combatants::CombatantProperties;
use crate::errors::AppError;
use crate::items::equipment::EquipmentProperties;
use rand::Rng;
use std::collections::HashMap;

pub fn calculate_weapon_swing_result(
    ability_user_id: u32,
    ability_target: &AbilityTarget,
    target_entity_id: u32,
    user_total_attributes: &HashMap<CombatAttributes, u16>,
    target_total_attributes: &HashMap<CombatAttributes, u16>,
    equipment_properties: &EquipmentProperties,
    is_off_hand: bool,
) -> Result<CombatActionResult, AppError> {
    let (weapon_properties, weapon_traits, base_bonus_damage) =
        get_weapon_properties_traits_and_base_bonus_damage(
            &user_total_attributes,
            &equipment_properties,
        )?;

    // determine hit or miss
    let accuracy = user_total_attributes
        .get(&CombatAttributes::Accuracy)
        .unwrap_or_else(|| &0);
    let (damage_range, weapon_hit_chance_before_defensive_mods) =
        CombatantProperties::get_weapon_damage_and_hit_chance(
            &weapon_properties,
            &weapon_traits,
            base_bonus_damage,
            *accuracy,
            is_off_hand,
        );
    let evaded = physical_attack_evaded(
        weapon_hit_chance_before_defensive_mods,
        target_total_attributes,
    );

    if evaded {
        Ok(CombatActionResult {
            user_id: ability_user_id,
            action: CombatAction::AbilityUsed(CombatantAbilityNames::Attack),
            targets: ability_target.clone(),
            hp_changes_by_entity_id: None,
            mp_changes_by_entity_id: None,
            misses_by_entity_id: Some(vec![(target_entity_id)]),
            resists_by_entity_id: None,
            is_crit: false,
            status_effect_changes_by_entity_id: None,
        })
    } else {
        let mut rng = rand::thread_rng();
        let rolled_damage = rng.gen_range(damage_range.min..=damage_range.max);

        // @TODO get crit chance and compare to target's crit avoidance stats
        // @TODO calculate blunt/piercing/slashing vs different armor types

        // compare rolled damage to defensive stats
        let final_damage = physical_damage_after_defense(
            user_total_attributes,
            target_total_attributes,
            rolled_damage as u32,
        );
        // add the result
        Ok(CombatActionResult {
            user_id: ability_user_id,
            action: CombatAction::AbilityUsed(CombatantAbilityNames::Attack),
            targets: ability_target.clone(),
            hp_changes_by_entity_id: Some(vec![IdAndValue(target_entity_id, final_damage as i16)]),
            mp_changes_by_entity_id: None,
            misses_by_entity_id: None,
            resists_by_entity_id: None,
            is_crit: false,
            status_effect_changes_by_entity_id: None,
        })
    }
}
