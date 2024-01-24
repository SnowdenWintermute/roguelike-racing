use super::combat_attributes::CombatAttributes;
use super::get_base_ability_damage_bonus::DamageSource;
use super::CombatantProperties;
use crate::app_consts::error_messages;
use crate::app_consts::TWO_HANDED_WEAPON_BASE_BONUS_DAMAGE_MODIFIER;
use crate::errors::AppError;
use crate::items::equipment::weapon_properties::WeaponProperties;
use crate::items::equipment::EquipmentProperties;
use crate::items::equipment::EquipmentTraits;
use crate::items::equipment::EquipmentTypes;
use std::collections::HashMap;

pub fn get_weapon_properties_traits_and_base_bonus_damage<'a>(
    total_attributes: &HashMap<CombatAttributes, u16>,
    equipment_properties: &'a EquipmentProperties,
) -> Result<(&'a WeaponProperties, &'a Option<Vec<EquipmentTraits>>, u16), AppError> {
    match &equipment_properties.equipment_type {
        EquipmentTypes::OneHandedMeleeWeapon(_, properties) => Ok((
            &properties,
            &equipment_properties.traits,
            CombatantProperties::get_base_ability_damage_bonus(
                &total_attributes,
                DamageSource::Melee,
            ),
        )),
        EquipmentTypes::TwoHandedMeleeWeapon(_, properties) => Ok((
            &properties,
            &equipment_properties.traits,
            CombatantProperties::get_base_ability_damage_bonus(
                &total_attributes,
                DamageSource::Melee,
            ) * TWO_HANDED_WEAPON_BASE_BONUS_DAMAGE_MODIFIER,
        )),
        EquipmentTypes::TwoHandedRangedWeapon(_, properties) => Ok((
            &properties,
            &equipment_properties.traits,
            CombatantProperties::get_base_ability_damage_bonus(
                &total_attributes,
                DamageSource::Ranged,
            ) * TWO_HANDED_WEAPON_BASE_BONUS_DAMAGE_MODIFIER,
        )),
        _ => {
            return Err(AppError {
                error_type: crate::errors::AppErrorTypes::Generic,
                message: error_messages::INVALID_EQUIPMENT_EQUIPPED.to_string(),
            })
        }
    }
}
