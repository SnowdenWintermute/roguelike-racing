#![allow(dead_code)]
use crate::game::id_generator::IdGenerator;
use crate::items::Item;
use crate::monster::Monster;
use rand::prelude::*;
use serde::{Deserialize, Serialize};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Debug, EnumIter, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum DungeonRoomTypes {
    MonsterLair,
    Treasure,
    Stairs,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TreasureChest {
    pub is_opened: bool,
    pub is_locked: bool,
    pub is_trapped: bool,
    pub level: u8,
}

impl TreasureChest {
    pub fn generate(level: u8) -> TreasureChest {
        let mut rng = rand::thread_rng();

        let mut is_locked = false;
        if rng.gen_range(1..=5) > 3 {
            is_locked = true;
        }

        let mut is_trapped = false;
        if rng.gen_range(1..=5) > 3 {
            is_trapped = true;
        }

        TreasureChest {
            is_opened: false,
            is_locked,
            is_trapped,
            level,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DungeonRoom {
    pub room_type: DungeonRoomTypes,
    pub treasure_chest: Option<TreasureChest>,
    pub items: Option<Vec<Item>>,
    pub monsters: Option<Vec<Monster>>,
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
            monsters = Some(vec![Monster::generate(id_generator, floor)]);
        }

        DungeonRoom {
            room_type,
            items: None,
            monsters,
            treasure_chest,
        }
    }
}
