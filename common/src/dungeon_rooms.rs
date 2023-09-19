#![allow(dead_code)]
use crate::equipment::Item;
use crate::monster::Monster;
use rand::prelude::*;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Debug, EnumIter, Clone, Copy, PartialEq)]
pub enum DungeonRoomTypes {
    MonsterLair,
    Treasure,
    Stairs,
}

#[derive(Debug)]
pub struct TreasureChest {
    is_opened: bool,
    is_locked: bool,
    is_trapped: bool,
    level: u8,
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

#[derive(Debug)]
pub struct DungeonRoom {
    pub room_type: DungeonRoomTypes,
    pub treasure_chest: Option<TreasureChest>,
    pub items: Option<Vec<Item>>,
    pub monster: Option<Monster>,
}

impl DungeonRoom {
    pub fn generate(
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

        let mut monster = None;
        if room_type == DungeonRoomTypes::MonsterLair {
            monster = Some(Monster::generate(floor));
        }

        DungeonRoom {
            room_type,
            items: None,
            monster,
            treasure_chest,
        }
    }
}
