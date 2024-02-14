use super::consumables::ConsumableProperties;
use super::consumables::ConsumableTypes;
use super::Item;
use super::ItemProperties;
use crate::combatants::combat_attributes::CombatAttributes;
use crate::primatives::EntityProperties;
use std::collections::HashMap;

impl Item {
    pub fn new(
        id: u32,
        name: String,
        item_level: u8,
        requirements: Option<HashMap<CombatAttributes, u8>>,
        item_properties: ItemProperties,
    ) -> Item {
        Item {
            entity_properties: EntityProperties { id, name },
            item_level,
            requirements,
            item_properties,
        }
    }
    pub fn create_consumable_by_type(id: u32, consumable_type: ConsumableTypes) -> Item {
        Item {
            entity_properties: EntityProperties {
                id,
                name: format!("{}", consumable_type),
            },
            item_level: 1,
            requirements: None,
            item_properties: ItemProperties::Consumable(ConsumableProperties {
                consumable_type,
                uses_remaining: 1,
            }),
        }
    }
}
