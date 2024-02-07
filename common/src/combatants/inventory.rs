use crate::app_consts::error_messages;
use crate::app_consts::CHARACTER_INVENTORY_DEFAULT_CAPACITY;
use crate::errors::AppError;
use crate::errors::AppErrorTypes;
use crate::items::consumables::ConsumableProperties;
use crate::items::equipment::weapon_properties::WeaponProperties;
use crate::items::Item;
use crate::items::ItemProperties;
use crate::primatives::WeaponSlot;
use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Inventory {
    pub items: Vec<Item>,
    pub capacity: u8,
    pub shards: u16,
    pub autoinjectors: u16,
}

impl Inventory {
    pub fn new() -> Inventory {
        Inventory {
            items: Vec::new(),
            capacity: CHARACTER_INVENTORY_DEFAULT_CAPACITY as u8,
            shards: 0,
            autoinjectors: 0,
        }
    }

    pub fn remove_item(&mut self, item_id: u32) -> Result<Item, AppError> {
        Item::remove_item_from_vec(&mut self.items, item_id)
    }

    pub fn get_item<'a>(&'a self, item_id: &u32) -> Result<&'a Item, AppError> {
        for item in &self.items {
            if item.entity_properties.id == *item_id {
                return Ok(&item);
            }
        }
        Err(AppError {
            error_type: crate::errors::AppErrorTypes::Generic,
            message: error_messages::INVALID_ITEM_ID.to_string(),
        })
    }

    pub fn get_item_mut<'a>(&'a mut self, item_id: &u32) -> Result<&'a mut Item, AppError> {
        for item in &mut self.items {
            if item.entity_properties.id == *item_id {
                return Ok(item);
            }
        }
        Err(AppError {
            error_type: crate::errors::AppErrorTypes::Generic,
            message: error_messages::INVALID_ITEM_ID.to_string(),
        })
    }

    pub fn get_consumable<'a>(
        &'a self,
        item_id: &u32,
    ) -> Result<&'a ConsumableProperties, AppError> {
        let consumable_in_inventory = self.get_item(&item_id)?;
        match &consumable_in_inventory.item_properties {
            ItemProperties::Consumable(conumable_properties) => Ok(conumable_properties),
            ItemProperties::Equipment(_) => Err(AppError {
                error_type: AppErrorTypes::InvalidInput,
                message: error_messages::CANT_CONSUME_NON_CONSUMABLE_ITEM.to_string(),
            }),
        }
    }

    pub fn get_consumable_mut<'a>(
        &'a mut self,
        item_id: &u32,
    ) -> Result<&'a mut ConsumableProperties, AppError> {
        let consumable_in_inventory = self.get_item_mut(&item_id)?;
        match &mut consumable_in_inventory.item_properties {
            ItemProperties::Consumable(conumable_properties) => Ok(conumable_properties),
            ItemProperties::Equipment(_) => Err(AppError {
                error_type: AppErrorTypes::InvalidInput,
                message: error_messages::CANT_CONSUME_NON_CONSUMABLE_ITEM.to_string(),
            }),
        }
    }
}
