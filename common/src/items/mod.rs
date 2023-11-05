#![allow(dead_code)]
pub mod affixes;
pub mod body_armor;
pub mod consumables;
pub mod equipment;
pub mod equipment_base_items;
mod generate_equipment_affixes;
pub mod generate_equipment_attributes;
mod generate_equipment_durability;
pub mod generate_equipment_properties_by_base_item;
mod select_random_affix_types;
mod weapons;
use self::consumables::ConsumableProperties;
use self::equipment::EquipmentProperties;
use self::generate_equipment_properties_by_base_item::generate_equipment_properties_by_base_item;
use crate::combatants::abilities::{TargetingScheme, ValidTargets};
use crate::game::id_generator::IdGenerator;
use crate::primatives::{EntityProperties, MaxAndCurrent};
use rand::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use strum_macros::EnumIter;
mod generate_consumable_properties;
mod generate_equipment_properties;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Eq)]
pub enum ItemCategories {
    Equipment,
    Consumable,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ItemProperties {
    Consumable(ConsumableProperties),
    Equipment(EquipmentProperties),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Item {
    pub entity_properties: EntityProperties,
    pub item_level: u8,
    pub item_category: ItemCategories,
    pub item_properties: ItemProperties,
}

const CHANCE_OF_CONSUMABLE_DROP: u16 = 20;

impl Item {
    pub fn generate(id_generator: &mut IdGenerator, level: u8) -> Item {
        let mut rng = rand::thread_rng();
        let random_number = rng.gen_range(1..100);
        let mut item_category: ItemCategories = ItemCategories::Equipment;
        if random_number < CHANCE_OF_CONSUMABLE_DROP {
            item_category = ItemCategories::Consumable;
        }

        // let item_properties = match item_category {
        //     // ItemCategories::Equipment => Item::generate_equipment_properties(level),
        //     ItemCategories::Consumable => Item::generate_consumable_properties(level),
        //     ItemCategories::Equipment => (),
        // };
        //
        let equipment_properties = generate_equipment_properties_by_base_item(level);

        Item {
            entity_properties: EntityProperties {
                id: id_generator.get_next_entity_id(),
                name: "some item name".to_owned(),
            },
            item_level: level as u8,
            item_category,
            item_properties: ItemProperties::Equipment(equipment_properties),
        }
    }
}
