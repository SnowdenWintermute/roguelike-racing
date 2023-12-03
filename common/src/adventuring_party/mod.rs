mod generate_next_room;

use crate::character::Character;
use crate::dungeon_rooms::DungeonRoom;
use crate::dungeon_rooms::DungeonRoomTypes;
use crate::items::Item;
use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RoomsExplored {
    pub total: u16,
    pub on_current_floor: u16,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AdventuringParty {
    pub id: u32,
    pub name: String,
    pub player_usernames: HashSet<String>,
    pub players_ready_to_explore: HashSet<String>,
    pub characters: HashMap<u32, Character>,
    pub character_positions: Vec<u32>,
    pub active_combatant_id: Option<u32>,
    pub current_floor: u8,
    pub rooms_explored: RoomsExplored,
    pub current_room: DungeonRoom,
    pub time_of_wipe: Option<u64>,
    pub time_of_escape: Option<u64>,
}

impl AdventuringParty {
    pub fn new(id: u32, name: String) -> AdventuringParty {
        AdventuringParty {
            id,
            name,
            player_usernames: HashSet::new(),
            players_ready_to_explore: HashSet::new(),
            characters: HashMap::new(),
            character_positions: Vec::new(),
            active_combatant_id: None,
            current_floor: 1,
            rooms_explored: RoomsExplored {
                total: 1,
                on_current_floor: 1,
            },
            current_room: DungeonRoom {
                room_type: DungeonRoomTypes::Empty,
                treasure_chest: None,
                items: None,
                monsters: None,
            },
            time_of_wipe: None,
            time_of_escape: None,
        }
    }

    pub fn get_item_by_id<'a>(&'a self, id: u32) -> Option<&'a Item> {
        for (_, character) in &self.characters {
            for (_, item) in &character.combatant_properties.equipment {
                if item.entity_properties.id == id {
                    return Some(item);
                }
            }
            for item in &character.inventory.items {
                if item.entity_properties.id == id {
                    return Some(item);
                }
            }
            if let Some(items) = &self.current_room.items {
                for item in items {
                    if item.entity_properties.id == id {
                        return Some(item);
                    }
                }
            }
            if let Some(monsters) = &self.current_room.monsters {
                for monster in monsters {
                    for (_, item) in &monster.combatant_properties.equipment {
                        if item.entity_properties.id == id {
                            return Some(item);
                        }
                    }
                }
            }
        }

        None
    }
}
