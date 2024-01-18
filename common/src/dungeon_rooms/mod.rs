mod treasure_chest;
use crate::dungeon_rooms::treasure_chest::TreasureChest;
use crate::game::id_generator::IdGenerator;
use crate::items::Item;
use crate::monsters::Monster;
use core::fmt;
use rand::prelude::*;
use serde::Deserialize;
use serde::Serialize;
use std::collections::hash_map::HashMap;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Debug, EnumIter, Clone, Copy, PartialEq, Serialize, Deserialize, Eq)]
pub enum DungeonRoomTypes {
    MonsterLair,
    Treasure,
    Stairs,
    Empty,
}

impl fmt::Display for DungeonRoomTypes {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DungeonRoomTypes::MonsterLair => write!(f, "Monster Lair"),
            DungeonRoomTypes::Treasure => write!(f, "Treasure Room"),
            DungeonRoomTypes::Stairs => write!(f, "Staircase"),
            DungeonRoomTypes::Empty => write!(f, "Empty"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DungeonRoom {
    pub room_type: DungeonRoomTypes,
    pub treasure_chest: Option<TreasureChest>,
    pub items: Option<Vec<Item>>,
    pub monsters: Option<HashMap<u32, Monster>>,
}

impl DungeonRoom {
    pub fn generate(
        id_generator: &mut IdGenerator,
        floor: u8,
        stairs_possible: bool,
        forced_type: Option<DungeonRoomTypes>,
    ) -> DungeonRoom {
        let room_type = match forced_type {
            Some(dungeon_room_type) => dungeon_room_type,
            None => {
                if stairs_possible {
                    let room_types: Vec<_> = DungeonRoomTypes::iter().collect();
                    *room_types.choose(&mut rand::thread_rng()).unwrap()
                } else {
                    let mut room_types: Vec<_> = DungeonRoomTypes::iter().collect();
                    room_types.retain(|&room_type| room_type != DungeonRoomTypes::Stairs);
                    *room_types.choose(&mut rand::thread_rng()).unwrap()
                }
            }
        };

        let mut treasure_chest = None;
        if room_type == DungeonRoomTypes::Treasure {
            treasure_chest = Some(TreasureChest::generate(floor));
        }

        let mut monsters = None;
        if room_type == DungeonRoomTypes::MonsterLair {
            let mut new_monsters = HashMap::new();
            for i in 0..=2 {
                let new_monster = Monster::generate(id_generator, floor, i * 10);
                new_monsters.insert(new_monster.entity_properties.id, new_monster);
            }
            monsters = Some(new_monsters);
        }

        DungeonRoom {
            room_type,
            items: None,
            monsters,
            treasure_chest,
        }
    }
}
