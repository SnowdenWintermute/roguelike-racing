use super::one_handed_melee_weapons::OneHandedMeleeWeapons;
use super::weapon_properties::DamageClassifications;
use super::weapon_properties::DamageTypes;
use super::weapon_properties::WeaponProperties;
use super::EquipmentProperties;
use super::EquipmentTypes;
use once_cell::sync::Lazy;
use std::collections::HashMap;

pub const FIST: Lazy<EquipmentProperties> = Lazy::new(|| EquipmentProperties {
    equipment_type: EquipmentTypes::OneHandedMeleeWeapon(
        OneHandedMeleeWeapons::Stick,
        WeaponProperties {
            damage_classifications: vec![DamageClassifications::Physical(DamageTypes::Blunt)],
            damage: crate::primatives::Range { min: 1, max: 1 },
        },
    ),
    durability: None,
    attributes: HashMap::new(),
    affixes: vec![],
    traits: None,
});
