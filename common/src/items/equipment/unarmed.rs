use super::one_handed_melee_weapons::OneHandedMeleeWeapons;
use super::weapon_properties::WeaponProperties;
use super::EquipmentProperties;
use super::EquipmentTypes;
use crate::combat::hp_change_source_types::HpChangeSource;
use crate::combat::hp_change_source_types::HpChangeSourceCategories;
use crate::combat::hp_change_source_types::HpChangeSourceSubCategories;
use once_cell::sync::Lazy;
use std::collections::HashMap;

pub static FIST: Lazy<EquipmentProperties> = Lazy::new(|| EquipmentProperties {
    equipment_type: EquipmentTypes::OneHandedMeleeWeapon(
        OneHandedMeleeWeapons::Stick,
        WeaponProperties {
            damage_classifications: vec![HpChangeSource {
                category: HpChangeSourceCategories::PhysicalDamage,
                sub_category: Some(HpChangeSourceSubCategories::Blunt),
                element: None,
            }],
            damage: crate::primatives::Range { min: 1, max: 1 },
        },
    ),
    durability: None,
    attributes: HashMap::new(),
    affixes: vec![],
    traits: None,
});
