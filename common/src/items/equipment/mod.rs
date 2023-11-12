pub mod affixes;
pub mod armor_properties;
pub mod body_armors;
mod display_equipment;
pub mod equipment_generation;
pub mod head_gears;
pub mod jewelries;
pub mod one_handed_melee_weapons;
pub mod shield_properties;
pub mod shields;
pub mod two_handed_melee_weapons;
pub mod two_handed_ranged_weapons;
pub mod weapon_properties;
use self::affixes::Affix;
use self::armor_properties::ArmorProperties;
use self::body_armors::BodyArmors;
use self::head_gears::HeadGears;
use self::one_handed_melee_weapons::OneHandedMeleeWeapons;
use self::shield_properties::ShieldProperties;
use self::shields::Shields;
use self::two_handed_melee_weapons::TwoHandedMeleeWeapons;
use self::two_handed_ranged_weapons::TwoHandedRangedWeapons;
use self::weapon_properties::WeaponProperties;
use crate::combatants::CombatAttributes;
use crate::primatives::MaxAndCurrent;
use core::fmt;
use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;
use strum_macros::EnumIter;

#[derive(Debug, Serialize, Deserialize, Clone, Hash, Eq, PartialEq)]
pub enum EquipmentSlots {
    LeftHand,
    RightHand,
    Head,
    Body,
    LeftRing,
    RightRing,
    Amulet,
}

#[derive(Debug, EnumIter, Clone, PartialEq, Serialize, Deserialize, Eq)]
pub enum EquipmentTypes {
    BodyArmor(BodyArmors, ArmorProperties),
    HeadGear(HeadGears, ArmorProperties),
    Ring,
    Amulet,
    OneHandedMeleeWeapon(OneHandedMeleeWeapons, WeaponProperties),
    TwoHandedMeleeWeapon(TwoHandedMeleeWeapons, WeaponProperties),
    TwoHandedRangedWeapon(TwoHandedRangedWeapons, WeaponProperties),
    Shield(Shields, ShieldProperties),
}

#[derive(Debug, EnumIter, Clone, Copy, PartialEq, Serialize, Deserialize, Eq)]
pub enum EquipmentTraits {
    LifeStealPercentage(u8),
    DurabilityBonus(u8),
    ArmorClassPercentage(u8),
    DamagePercentage(u8),
    RandomDamageTypeSelection,
}

impl fmt::Display for EquipmentTraits {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            EquipmentTraits::LifeStealPercentage(number) => {
                write!(f, "Life steal percentage: {}", number)
            }
            EquipmentTraits::DurabilityBonus(number) => write!(f, "Durability bonus: {}", number),
            EquipmentTraits::ArmorClassPercentage(number) => {
                write!(f, "Bonus armor class percentage: {}", number)
            }
            EquipmentTraits::DamagePercentage(number) => {
                write!(f, "Bonus damage percentage: {}", number)
            }
            EquipmentTraits::RandomDamageTypeSelection => {
                write!(f, "Damage classification selected randomly")
            }
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct EquipmentProperties {
    pub equipment_type: EquipmentTypes,
    pub durability: Option<MaxAndCurrent<u8>>,
    pub attributes: HashMap<CombatAttributes, u16>,
    pub affixes: Vec<Affix>,
    pub requirements: HashMap<CombatAttributes, u8>,
    pub traits: Option<Vec<EquipmentTraits>>,
}