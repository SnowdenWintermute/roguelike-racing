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
pub mod trait_effects;
pub mod two_handed_melee_weapons;
pub mod two_handed_ranged_weapons;
pub mod unarmed;
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
use crate::combatants::combat_attributes::CombatAttributes;
use crate::primatives::MaxAndCurrent;
use core::fmt;
use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;
use strum_macros::EnumIter;

#[derive(Debug, Serialize, Deserialize, Clone, Hash, Eq, PartialEq)]
pub enum EquipmentSlots {
    MainHand,
    OffHand,
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

impl fmt::Display for EquipmentTypes {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            EquipmentTypes::BodyArmor(_, _) => write!(f, "Body Armor"),
            EquipmentTypes::HeadGear(_, _) => write!(f, "Head Gear"),
            EquipmentTypes::Ring => write!(f, "Ring"),
            EquipmentTypes::Amulet => write!(f, "Amulet"),
            EquipmentTypes::OneHandedMeleeWeapon(_, _) => write!(f, "One Handed Melee Weapon"),
            EquipmentTypes::TwoHandedMeleeWeapon(_, _) => write!(f, "Two Handed Melee Weapon"),
            EquipmentTypes::TwoHandedRangedWeapon(_, _) => write!(f, "Two Handed Ranged Weapon"),
            EquipmentTypes::Shield(_, _) => write!(f, "Shield"),
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
    pub traits: Option<Vec<EquipmentTraits>>,
}

impl EquipmentProperties {
    pub fn get_base_armor_class(&self) -> u8 {
        match &self.equipment_type {
            EquipmentTypes::BodyArmor(_, armor_properties) => armor_properties.armor_class,
            EquipmentTypes::HeadGear(_, armor_properties) => armor_properties.armor_class,
            EquipmentTypes::Shield(_, shield_properties) => shield_properties.armor_class,
            _ => 0,
        }
    }
}

#[derive(PartialEq, Clone, Debug)]
pub struct EquipableSlots {
    pub main: EquipmentSlots,
    pub alternate: Option<EquipmentSlots>,
}

impl EquipableSlots {
    pub fn new(main: EquipmentSlots, alternate: Option<EquipmentSlots>) -> Self {
        EquipableSlots { main, alternate }
    }
}

impl EquipmentProperties {
    pub fn get_equippable_slots(&self) -> EquipableSlots {
        match self.equipment_type {
            EquipmentTypes::BodyArmor(_, _) => EquipableSlots::new(EquipmentSlots::Body, None),
            EquipmentTypes::HeadGear(_, _) => EquipableSlots::new(EquipmentSlots::Head, None),
            EquipmentTypes::Ring => {
                EquipableSlots::new(EquipmentSlots::RightRing, Some(EquipmentSlots::LeftRing))
            }

            EquipmentTypes::Amulet => EquipableSlots::new(EquipmentSlots::Amulet, None),
            EquipmentTypes::OneHandedMeleeWeapon(_, _) => {
                EquipableSlots::new(EquipmentSlots::MainHand, Some(EquipmentSlots::OffHand))
            }

            EquipmentTypes::TwoHandedMeleeWeapon(_, _) => {
                EquipableSlots::new(EquipmentSlots::MainHand, None)
            }
            EquipmentTypes::TwoHandedRangedWeapon(_, _) => {
                EquipableSlots::new(EquipmentSlots::MainHand, None)
            }
            EquipmentTypes::Shield(_, _) => EquipableSlots::new(EquipmentSlots::OffHand, None),
        }
    }

    pub fn is_two_handed(&self) -> bool {
        match self.equipment_type {
            EquipmentTypes::TwoHandedMeleeWeapon(_, _)
            | EquipmentTypes::TwoHandedRangedWeapon(_, _) => true,
            _ => false,
        }
    }
}
