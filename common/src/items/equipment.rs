use crate::combatants::CombatAttributes;
use crate::primatives::MaxAndCurrent;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use strum_macros::EnumIter;

use super::{
    armors::{ArmorCategories, Armors},
    weapons::PhysicalDamageTypes,
};

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

#[derive(Debug, EnumIter, Clone, Copy, PartialEq, Serialize, Deserialize, Eq)]
pub enum EquipmentTypes {
    BodyArmor(Armors, ArmorCategories),
    Helmet(ArmorCategories),
    Ring,
    Amulet,
    OneHandedWeapon(PhysicalDamageTypes),
    TwoHandedWeapon(PhysicalDamageTypes),
    RangedWeapon(PhysicalDamageTypes),
    Shield(ShieldTypes),
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
    pub requirements: HashMap<CombatAttributes, u16>,
}
