use super::CombatantProperties;
use crate::items::{
    equipment::{
        weapon_properties::WeaponProperties, EquipmentSlots, EquipmentTraits, EquipmentTypes,
    },
    ItemProperties,
};

impl CombatantProperties {
    pub fn get_equipped_weapon_properties(
        &self,
        slot: &EquipmentSlots,
    ) -> Option<(&WeaponProperties, &Option<Vec<EquipmentTraits>>)> {
        match self.equipment.get(slot) {
            Some(item) => match &item.item_properties {
                ItemProperties::Consumable(_) => None,
                ItemProperties::Equipment(properties) => match &properties.equipment_type {
                    EquipmentTypes::OneHandedMeleeWeapon(_, weapon_properties)
                    | EquipmentTypes::TwoHandedMeleeWeapon(_, weapon_properties)
                    | EquipmentTypes::TwoHandedRangedWeapon(_, weapon_properties) => {
                        Some((&weapon_properties, &properties.traits))
                    }
                    _ => None,
                },
            },
            None => None,
        }
    }
}
