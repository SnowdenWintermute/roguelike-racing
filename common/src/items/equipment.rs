use super::body_armor::{ArmorCategories, BodyArmors};
use super::headgear::HeadGears;
use super::one_handed_melee_weapons::{DamageClassifications, DamageTypes, OneHandedMeleeWeapons};
use crate::items::affixes::Affix;
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
    BodyArmor(BodyArmors, ArmorCategories),
    HeadGear(HeadGears, ArmorCategories),
    Ring,
    Amulet,
    OneHandedMeleeWeapon(OneHandedMeleeWeapons, Vec<DamageClassifications>),
    TwoHandedWeapon(DamageClassifications),
    RangedWeapon(DamageClassifications),
    Shield(ShieldTypes),
}

impl fmt::Display for EquipmentTypes {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            EquipmentTypes::BodyArmor(base_armor, category) => {
                write!(f, "{base_armor} ({category})")
            }
            EquipmentTypes::HeadGear(_, _) => write!(f, ""),
            EquipmentTypes::Ring => write!(f, ""),
            EquipmentTypes::Amulet => write!(f, ""),
            EquipmentTypes::OneHandedMeleeWeapon(base_item, damage_classifications) => {
                write!(f, "{base_item} {:?}", damage_classifications)
            }
            EquipmentTypes::TwoHandedWeapon(_) => write!(f, ""),
            EquipmentTypes::RangedWeapon(_) => write!(f, ""),
            EquipmentTypes::Shield(_) => write!(f, ""),
        }
    }
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
    pub base_ac: Option<u8>,
    pub base_damage: Option<Range<u8>>,
    pub attributes: HashMap<CombatAttributes, u16>,
    pub affixes: Vec<Affix>,
    pub requirements: HashMap<CombatAttributes, u8>,
}
