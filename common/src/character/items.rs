use serde::{Deserialize, Serialize};

use crate::{app_consts::CHARACTER_INVENTORY_DEFAULT_CAPACITY, items::Item};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CombatantEquipment {
    left_hand: Option<Item>,
    right_hand: Option<Item>,
    head: Option<Item>,
    body: Option<Item>,
    left_ring: Option<Item>,
    right_ring: Option<Item>,
    amulet: Option<Item>,
}

impl CombatantEquipment {
    pub fn new() -> CombatantEquipment {
        CombatantEquipment {
            left_hand: None,
            right_hand: None,
            head: None,
            body: None,
            left_ring: None,
            right_ring: None,
            amulet: None,
        }
    }
}

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
