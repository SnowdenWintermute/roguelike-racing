pub mod affixes;
pub mod armor_properties;
pub mod body_armors;
pub mod equipment_generation;
pub mod head_gears;
pub mod jewelries;
pub mod one_handed_melee_weapons;
pub mod pre_made_items;
pub mod shield_properties;
pub mod shields;
pub mod trait_effects;
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
use crate::app_consts::error_messages;
use crate::combatants::combat_attributes::CombatAttributes;
use crate::errors::AppError;
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
            EquipmentTraits::LifeStealPercentage(value) => {
                write!(f, "{value}% lifesteal")
            }
            EquipmentTraits::DurabilityBonus(_) => write!(f, "Increased durability"),
            EquipmentTraits::ArmorClassPercentage(value) => write!(f, "+{value}% armor class"),
            EquipmentTraits::DamagePercentage(value) => write!(f, "+{value}% weapon damage"),
            EquipmentTraits::RandomDamageTypeSelection => {
                write!(f, "Damage type selected randomly")
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
    pub fn new(
        equipment_type: EquipmentTypes,
        durability: Option<MaxAndCurrent<u8>>,
        attributes: HashMap<CombatAttributes, u16>,
        affixes: Vec<Affix>,
        traits: Option<Vec<EquipmentTraits>>,
    ) -> Self {
        EquipmentProperties {
            equipment_type,
            durability,
            attributes,
            affixes,
            traits,
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
    pub fn get_base_armor_class(&self) -> u8 {
        match &self.equipment_type {
            EquipmentTypes::BodyArmor(_, armor_properties) => armor_properties.armor_class,
            EquipmentTypes::HeadGear(_, armor_properties) => armor_properties.armor_class,
            EquipmentTypes::Shield(_, shield_properties) => shield_properties.armor_class,
            _ => 0,
        }
    }

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

    pub fn is_weapon(&self) -> bool {
        match self.equipment_type {
            EquipmentTypes::OneHandedMeleeWeapon(_, _)
            | EquipmentTypes::TwoHandedMeleeWeapon(_, _)
            | EquipmentTypes::TwoHandedRangedWeapon(_, _) => true,
            _ => false,
        }
    }

    pub fn is_shield(equipment_properties_option: Option<&EquipmentProperties>) -> bool {
        match equipment_properties_option {
            Some(equipment_properties) => match equipment_properties.equipment_type {
                EquipmentTypes::Shield(_, _) => true,
                _ => false,
            },
            None => false,
        }
    }

    pub fn is_two_handed(&self) -> bool {
        match self.equipment_type {
            EquipmentTypes::TwoHandedMeleeWeapon(_, _)
            | EquipmentTypes::TwoHandedRangedWeapon(_, _) => true,
            _ => false,
        }
    }

    pub fn get_weapon_equipment_properties_option_from_equipment_properties_option(
        equipment_properties_option: Option<&EquipmentProperties>,
    ) -> Option<&EquipmentProperties> {
        match equipment_properties_option {
            Some(equipment_properties) => match equipment_properties.is_weapon() {
                true => Some(equipment_properties),
                false => None,
            },
            None => None,
        }
    }

    pub fn get_equipment_weapon_properties(&self) -> Result<&WeaponProperties, AppError> {
        match &self.equipment_type {
            EquipmentTypes::OneHandedMeleeWeapon(_, weapon_properties)
            | EquipmentTypes::TwoHandedMeleeWeapon(_, weapon_properties)
            | EquipmentTypes::TwoHandedRangedWeapon(_, weapon_properties) => Ok(&weapon_properties),
            _ => {
                return Err(AppError {
                    error_type: crate::errors::AppErrorTypes::Generic,
                    message: error_messages::INVALID_EQUIPMENT_SLOT.to_string(),
                })
            }
        }
    }

    pub fn get_modified_weapon_damage_range(&self) -> Result<(f32, f32), AppError> {
        let weapon_properties = self.get_equipment_weapon_properties()?;
        let damage_attribute_bonus = *self
            .attributes
            .get(&CombatAttributes::Damage)
            .unwrap_or_else(|| &0) as f32;
        let percent_weapon_damage_modifier =
            self.get_weapon_percent_damage_increase_trait_damage_modifier();
        let weapon_min = (weapon_properties.damage.min as f32 + damage_attribute_bonus)
            * percent_weapon_damage_modifier;
        let weapon_max = (weapon_properties.damage.max as f32 + damage_attribute_bonus)
            * percent_weapon_damage_modifier;
        Ok((weapon_min, weapon_max))
    }
}
