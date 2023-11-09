use crate::app_consts::error_messages;
use crate::app_consts::MAX_PARTY_SIZE;
use crate::character::Character;
use crate::combatants::CombatantClass;
use crate::dungeon_rooms::DungeonRoom;
use crate::dungeon_rooms::DungeonRoomTypes;
use crate::errors::AppError;
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
    pub characters: HashMap<u32, Character>,
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
            characters: HashMap::new(),
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

    // pub fn add_player_character(
    //     &mut self,
    //     id: u32,
    //     combatant_class: CombatantClass,
    //     name: &str,
    //     name_of_controlling_user: String,
    // ) -> Result<(), AppError> {
    //     if self.characters.len() >= MAX_PARTY_SIZE.into() {
    //         return Err(AppError {
    //             error_type: crate::errors::AppErrorTypes::InvalidInput,
    //             message: error_messages::PARTY_FULL.to_string(),
    //         });
    //     }
    //     let new_character = Character::new(id, name, combatant_class, name_of_controlling_user);
    //     self.characters
    //         .insert(new_character.entity_properties.id, new_character);
    //     Ok(())
    // }
}
