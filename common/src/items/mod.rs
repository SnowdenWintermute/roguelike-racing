#![allow(dead_code)]
use crate::character::abilities::{TargetingScheme, ValidTargets};
use crate::game::id_generator::IdGenerator;
use crate::primatives::{EntityProperties, MaxAndCurrent};
use rand::prelude::*;
use serde::{Deserialize, Serialize};
use strum_macros::EnumIter;
mod generate_consumable_properties;
mod generate_equipment_properties;

#[derive(Debug, EnumIter, Clone, Copy)]
pub enum StatTypes {
    Dexterity,
    Strength,
    Intelligence,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum ItemCategories {
    Equipment,
    Consumable,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum EquipmentSlots {
    LeftHand,
    RightHand,
    Head,
    Body,
    LeftRing,
    RightRing,
    Amulet,
}

#[derive(Debug, EnumIter, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum EquipmentTypes {
    BodyArmor,
    Helmet,
    Ring,
    Amulet,
    OneHandedWeapon,
    TwoHandedWeapon,
    Shield,
}

#[derive(Debug, EnumIter, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum ConsumableTypes {
    RoomFinder,
    RepairKit,
    UpgradeKit,
    SmokeBomb,
    MilkDrink,
    FruitDrink,
    MonsterScanner,
    Antidote,
    Grenade,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct EquipmentProperties {
    pub equipment_type: EquipmentTypes,
    pub damage: u16,
    pub armor_class: u16,
    pub durability: Option<MaxAndCurrent<u16>>,
    pub dexterity: u16,
    pub strength: u16,
    pub intelligence: u16,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ConsumableProperties {
    pub consumable_type: ConsumableTypes,
    pub uses_remaining: u8,
    pub combat_use_only: bool,
    pub requires_combat_turn: bool,
    pub targeting_schemes: Vec<TargetingScheme>,
    pub valid_targets: ValidTargets,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ItemProperties {
    Consumable(ConsumableProperties),
    Equipment(EquipmentProperties),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Item {
    pub entity_properties: EntityProperties,
    pub item_level: u8,
    pub item_category: ItemCategories,
    pub item_properties: ItemProperties,
}

impl Item {
    pub fn generate(id_generator: &mut IdGenerator, level: u16) -> Item {
        let mut rng = rand::thread_rng();
        let random_number = rng.gen_range(1..6);
        let mut item_category: ItemCategories = ItemCategories::Equipment;
        if random_number > 5 {
            item_category = ItemCategories::Consumable;
        }

        let item_properties = match item_category {
            ItemCategories::Equipment => Item::generate_equipment_properties(level),
            ItemCategories::Consumable => Item::generate_consumable_properties(level),
        };

        Item {
            entity_properties: EntityProperties {
                id: id_generator.get_next_entity_id(),
                name: "some item name".to_owned(),
            },
            item_level: level as u8,
            item_category,
            item_properties,
        }
    }
}
