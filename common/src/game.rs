#![allow(dead_code)]
use std::collections::HashMap;

use crate::character::{Character, CharacterClasses};
use crate::equipment::Item;
use crate::monster::Monster;
use rand::prelude::*;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Debug)]
pub struct User {
    email: String,
}

#[derive(Debug)]
pub struct Game {
    player_characters: HashMap<String, Character>,
    last_assigned_entity_id: u32,
}

impl Game {
    pub fn new() -> Game {
        Game {
            player_characters: HashMap::new(),
            last_assigned_entity_id: 0,
        }
    }

    pub fn add_player_character(
        &mut self,
        user_email: &str,
        character_class: CharacterClasses,
    ) -> () {
        self.player_characters.insert(
            user_email.to_string(),
            Character::new(user_email.to_string(), character_class),
        );
    }

    pub fn get_next_entity_id(&mut self) -> u32 {
        self.last_assigned_entity_id += 1;
        self.last_assigned_entity_id
    }
}
