#![allow(dead_code)]
use std::collections::HashMap;

use crate::consts::DEEPEST_FLOOR;
use crate::dungeon_rooms::{DungeonRoom, DungeonRoomTypes};
use crate::errors::{AppError, AppErrorTypes};
use crate::game::IdGenerator;
use crate::primatives::{EntityProperties, MaxAndCurrent, UpOrDown};
use crate::status_effects::StatusEffects;

use self::abilities::{CharacterAbilities, CharacterAbility};
use self::items::{CharacterEquipment, CharacterInventory};

pub mod abilities;
pub mod items;

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
pub struct Character {
    pub entity_properties: EntityProperties,
    pub user_email: String,
    pub hit_points: MaxAndCurrent<u16>,
    pub mana: MaxAndCurrent<u16>,
    pub status_effects: Vec<StatusEffects>,
    pub equipment: CharacterEquipment,
    pub inventory: CharacterInventory,
    pub abilities: HashMap<CharacterAbilities, CharacterAbility>,
    pub unspent_ability_points: u8,
    pub current_floor: u8,
    pub rooms_explored: RoomsExplored,
    pub current_room: DungeonRoom,
}

impl Character {
    pub fn new(
        id_generator: &mut IdGenerator,
        name: &str,
        user_email: String,
        character_class: CharacterClasses,
    ) -> Character {
        let mut abilities = HashMap::<CharacterAbilities, CharacterAbility>::new();
        abilities.insert(
            CharacterAbilities::Attack,
            CharacterAbilities::new(&CharacterAbilities::Attack),
        );
        match character_class {
            CharacterClasses::Mage => {
                abilities.insert(
                    CharacterAbilities::HeatLance,
                    CharacterAbilities::new(&CharacterAbilities::HeatLance),
                );
            }
            CharacterClasses::Rogue => {
                abilities.insert(
                    CharacterAbilities::ShootArrow,
                    CharacterAbilities::new(&CharacterAbilities::ShootArrow),
                );
            }
            CharacterClasses::Warrior => {
                abilities.insert(
                    CharacterAbilities::ArmorBreak,
                    CharacterAbilities::new(&CharacterAbilities::ArmorBreak),
                );
            }
        }

        Character {
            entity_properties: EntityProperties {
                id: id_generator.get_next_entity_id(),
                name: name.to_owned(),
            },
            user_email,
            hit_points: MaxAndCurrent::new(10, 10),
            mana: MaxAndCurrent::new(10, 10),
            status_effects: vec![],
            equipment: CharacterEquipment::new(),
            inventory: CharacterInventory::new(),
            abilities,
            unspent_ability_points: 1,
            current_floor: 1,
            rooms_explored: RoomsExplored {
                total: 0,
                on_current_floor: 0,
            },
            current_room: DungeonRoom::generate(
                id_generator,
                1,
                true,
                Some(DungeonRoomTypes::Stairs),
            ),
        }
    }

    pub fn explore_dungeon(&mut self, id_generator: &mut IdGenerator) -> Result<(), AppError> {
        if self.current_room.monster.is_some() {
            return Err(AppError {
                error_type: AppErrorTypes::InvalidInput,
                message: "The monster in this room is blocking you from exploring the next room."
                    .to_string(),
            });
        }

        self.rooms_explored.total += 1;
        self.rooms_explored.on_current_floor += 1;

        let possible_to_find_stairs = self.rooms_explored.on_current_floor > 3;
        self.current_room = DungeonRoom::generate(
            id_generator,
            self.current_floor,
            possible_to_find_stairs,
            None,
        );
        println!("generated room: ");
        println!("{:#?}", self.current_room);
        Ok(())
    }

    pub fn take_stairs(&mut self, id_generator: &mut IdGenerator, direction: UpOrDown) {
        if direction == UpOrDown::Down {
            self.current_floor += 1;
            if self.current_floor >= DEEPEST_FLOOR {
                return println!("escaped the dungeon");
            }
        } else {
            self.current_floor -= 1;
        }

        let _ = self.explore_dungeon(id_generator);
    }
}
