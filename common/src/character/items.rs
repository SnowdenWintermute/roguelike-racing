use crate::{consts::CHARACTER_INVENTORY_DEFAULT_CAPACITY, items::Item};

#[derive(Debug)]
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

#[derive(Debug)]
pub struct CharacterInventory {
    items: Vec<Item>,
    capacity: u8,
    shards: u16,
    autoinjectors: u16,
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
