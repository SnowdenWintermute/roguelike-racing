use crate::app_consts::CHARACTER_INVENTORY_DEFAULT_CAPACITY;
use crate::items::Item;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CharacterInventory {
    pub items: Vec<Item>,
    pub capacity: u8,
    pub shards: u16,
    pub autoinjectors: u16,
}

impl CharacterInventory {
    pub fn new() -> CharacterInventory {
        CharacterInventory {
            items: Vec::new(),
            capacity: CHARACTER_INVENTORY_DEFAULT_CAPACITY as u8,
            shards: 0,
            autoinjectors: 0,
        }
    }
}

