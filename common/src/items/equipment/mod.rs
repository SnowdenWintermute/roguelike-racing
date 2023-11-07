pub mod affixes;
pub mod armor;
pub mod body_armors;
pub mod equipment_generation;
pub mod head_gears;
pub mod one_handed_melee_weapons;
pub mod weapons;
use self::affixes::Affix;
use self::armor::ArmorProperties;
use self::body_armors::BodyArmors;
use self::head_gears::HeadGears;
use self::one_handed_melee_weapons::OneHandedMeleeWeapons;
use self::weapons::WeaponProperties;
use crate::primatives::MaxAndCurrent;
use crate::{combatants::CombatAttributes, primatives::Range};
use core::fmt;
use serde::{Deserialize, Serialize};
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
    TwoHandedWeapon(),
    RangedWeapon(),
    Shield(ShieldTypes),
}

impl fmt::Display for EquipmentTypes {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            EquipmentTypes::BodyArmor(base_item, properties) => {
                write!(f, "{} ({})", base_item, properties.armor_category)
            }
            EquipmentTypes::HeadGear(base_item, properties) => {
                write!(f, "{}, ({})", base_item, properties.armor_category)
            }
            EquipmentTypes::Ring => write!(f, ""),
            EquipmentTypes::Amulet => write!(f, ""),
            EquipmentTypes::OneHandedMeleeWeapon(base_item, damage_classifications) => {
                write!(f, "{base_item} {:?}", damage_classifications)
            }
            EquipmentTypes::TwoHandedWeapon() => write!(f, ""),
            EquipmentTypes::RangedWeapon() => write!(f, ""),
            EquipmentTypes::Shield(_) => write!(f, ""),
        }
    }
}

#[derive(Debug, EnumIter, Clone, Copy, PartialEq, Serialize, Deserialize, Eq)]
pub enum EquipmentTraits {
    LifeStealPercentage(u8),
    DurabilityBonus(u8),
    ArmorClassPercentage(u8),
    DamagePercentage(u8),
    RandomDamageTypeSelection,
}

#[derive(Debug, EnumIter, Clone, Copy, PartialEq, Serialize, Deserialize, Eq, Default)]
pub enum ShieldTypes {
    #[default]
    Buckler,
    Kite,
    Tower,
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
