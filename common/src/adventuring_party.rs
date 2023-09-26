use std::collections::HashMap;
use std::time::Instant;

use crate::character::{combatant_properties::CombatantClass, Character};
use crate::consts::MAX_PARTY_SIZE;
use crate::dungeon_rooms::{DungeonRoom, DungeonRoomTypes};
use crate::errors::AppError;
use crate::game::id_generator::IdGenerator;

#[derive(Debug)]
pub struct RoomsExplored {
    pub total: u16,
    pub on_current_floor: u16,
}

#[derive(Debug)]
pub struct AdventuringParty {
    pub id: u32,
    pub player_characters: HashMap<u32, Character>,
    pub active_player_id: Option<u32>,
    pub current_floor: u8,
    pub rooms_explored: RoomsExplored,
    pub current_room: Option<DungeonRoom>,
    pub time_of_death: Option<Instant>,
    pub time_of_escape: Option<Instant>,
}

impl AdventuringParty {
    pub fn new(id: u32) -> AdventuringParty {
        AdventuringParty {
            id,
            active_player_id: None,
            player_characters: HashMap::new(),
            current_floor: 1,
            rooms_explored: RoomsExplored {
                total: 0,
                on_current_floor: 0,
            },
            current_room: None,
            time_of_death: None,
            time_of_escape: None,
        }
    }

    pub fn add_player_character(
        &mut self,
        id_generator: &mut IdGenerator,
        combatant_class: CombatantClass,
        name: &str,
    ) -> Result<u32, AppError> {
        if self.player_characters.len() > MAX_PARTY_SIZE.into() {
            return Err(AppError {
                error_type: crate::errors::AppErrorTypes::InvalidInput,
                message: "failed to add a new character because doing so would exceed the maximum party size".to_string()
            });
        }
        let new_character = Character::new(id_generator, name, combatant_class);
        let new_character_id = new_character.entity_properties.id;
        self.player_characters
            .insert(new_character.entity_properties.id, new_character);
        Ok(new_character_id)
    }
}
