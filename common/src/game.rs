#![allow(dead_code)]
use std::collections::HashMap;

use crate::character::{Character, CharacterClasses};

#[derive(Debug)]
pub struct User {
    email: String,
}

pub struct IdGenerator {
    pub last_assigned_entity_id: u32,
}

impl IdGenerator {
    pub fn get_next_entity_id(&mut self) -> u32 {
        self.last_assigned_entity_id += 1;
        self.last_assigned_entity_id
    }
}

#[derive(Debug)]
pub struct Game {
    pub player_characters: HashMap<String, Character>,
}

impl Game {
    pub fn new() -> Game {
        Game {
            player_characters: HashMap::new(),
        }
    }

    pub fn add_player_character(
        &mut self,
        id_generator: &mut IdGenerator,
        user_email: &str,
        character_class: CharacterClasses,
    ) -> () {
        let new_character = Character::new(
            id_generator,
            "player name",
            user_email.to_string(),
            character_class,
        );
        self.player_characters
            .insert(user_email.to_string(), new_character);
    }
}
