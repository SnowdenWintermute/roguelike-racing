#![allow(dead_code)]
use crate::consts::{CHARACTER_INVENTORY_DEFAULT_CAPACITY, DEEPEST_FLOOR};
use crate::dungeon_rooms::{DungeonRoom, DungeonRoomTypes};
use crate::equipment::Item;
use crate::primatives::{MaxAndCurrent, UpOrDown};

#[derive(Debug)]
pub struct CharacterEquipment {
    left_hand: Option<Item>,
    right_hand: Option<Item>,
    head: Option<Item>,
    body: Option<Item>,
    left_ring: Option<Item>,
    right_ring: Option<Item>,
    amulet: Option<Item>,
}

impl CharacterEquipment {
    pub fn new() -> CharacterEquipment {
        CharacterEquipment {
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

#[derive(Debug)]
pub struct RoomsExplored {
    pub total: u16,
    pub on_current_floor: u16,
}

#[derive(Debug)]
pub enum CharacterClasses {
    Warrior,
    Mage,
    Rogue,
}

#[derive(Debug)]
pub struct CharacterAbility {
    name: String,
    class: Option<CharacterClasses>,
    level: u8,
}

#[derive(Debug)]
pub enum CharacterAbilities {
    Attack,
    HeatLance,
    ArmorBreak,
    ShootArrow,
}

impl CharacterAbilities {
    fn new(&self) -> CharacterAbility {
        match self {
            CharacterAbilities::Attack => CharacterAbility {
                name: "Attack".to_string(),
                class: None,
                level: 1,
            },
            CharacterAbilities::HeatLance => CharacterAbility {
                name: "Heat Lance".to_string(),
                class: Some(CharacterClasses::Mage),
                level: 0,
            },
            CharacterAbilities::ArmorBreak => CharacterAbility {
                name: "Armor Break".to_string(),
                class: Some(CharacterClasses::Warrior),
                level: 0,
            },
            CharacterAbilities::ShootArrow => CharacterAbility {
                name: "Shoot Arrow".to_string(),
                class: Some(CharacterClasses::Rogue),
                level: 0,
            },
        }
    }
}

#[derive(Debug)]
pub struct Character {
    pub user_email: String,
    pub hit_points: MaxAndCurrent<u16>,
    pub mana: MaxAndCurrent<u16>,
    pub equipment: CharacterEquipment,
    pub inventory: CharacterInventory,
    pub abilities: Vec<CharacterAbility>,
    pub unspent_ability_points: u8,
    pub current_floor: u8,
    pub rooms_explored: RoomsExplored,
    pub current_room: DungeonRoom,
}

impl Character {
    pub fn new(user_email: String, character_class: CharacterClasses) -> Character {
        let mut abilities = vec![CharacterAbilities::new(&CharacterAbilities::Attack)];
        match character_class {
            CharacterClasses::Mage => {
                abilities.push(CharacterAbilities::new(&CharacterAbilities::HeatLance));
            }
            CharacterClasses::Rogue => {
                abilities.push(CharacterAbilities::new(&CharacterAbilities::ShootArrow));
            }
            CharacterClasses::Warrior => {
                abilities.push(CharacterAbilities::new(&CharacterAbilities::ArmorBreak));
            }
        }

        Character {
            user_email,
            hit_points: MaxAndCurrent::new(10, 10),
            mana: MaxAndCurrent::new(10, 10),
            equipment: CharacterEquipment::new(),
            inventory: CharacterInventory::new(),
            abilities,
            unspent_ability_points: 1,
            current_floor: 1,
            rooms_explored: RoomsExplored {
                total: 0,
                on_current_floor: 0,
            },
            current_room: DungeonRoom::generate(1, true, Some(DungeonRoomTypes::Stairs)),
        }
    }

    pub fn explore_dungeon(&mut self) {
        if self.current_room.monster.is_some() {
            return ();
        }

        self.rooms_explored.total += 1;
        self.rooms_explored.on_current_floor += 1;

        let possible_to_find_stairs = self.rooms_explored.on_current_floor > 3;
        self.current_room = DungeonRoom::generate(self.current_floor, possible_to_find_stairs, None)
    }

    pub fn take_stairs(&mut self, direction: UpOrDown) {
        if direction == UpOrDown::Down {
            self.current_floor += 1;
            if self.current_floor >= DEEPEST_FLOOR {
                return println!("escaped the dungeon");
            }
        } else {
            self.current_floor -= 1;
        }

        self.explore_dungeon();
    }

    // pub fn tick_combat(&mut self)
}
