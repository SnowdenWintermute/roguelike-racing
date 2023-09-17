#![allow(dead_code)]
use crate::equipment::*;

pub struct User {
    email: String,
}

#[derive(Debug)]
pub struct EntityProperties {
    pub id: u32,
    pub name: String,
}

#[derive(Debug)]
pub struct MaxAndCurrent<T> {
    pub max: T,
    pub current: T,
}

impl<T> MaxAndCurrent<T> {
    fn new(&self, max: T, current: T) -> MaxAndCurrent<T> {
        MaxAndCurrent { max, current }
    }
}

pub struct CharacterEquipment {
    // hand_left: HoldableEquipment,
}

pub struct CharacterInventory {
    items: Vec<Item>,
    capacity: i8,
}

pub struct Character {
    owner: User,
    hit_points: MaxAndCurrent<u16>,
    mana: MaxAndCurrent<u16>,
    equipment: CharacterEquipment,
}

pub struct Game {
    users: (Option<User>, Option<User>),
    player_characters: (Option<Character>, Option<Character>),
    last_assigned_entity_id: u32,
}

impl Game {
    pub fn get_next_entity_id(&self) -> u32 {
        self.last_assigned_entity_id += 1;
        self.last_assigned_entity_id
    }
}
