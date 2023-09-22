#![allow(dead_code)]
use crate::adventuring_party::AdventuringParty;
use crate::game::id_generator::IdGenerator;
use std::{collections::HashMap, time::Instant};

use self::player_actions::PlayerActionRequest;
pub mod id_generator;
pub mod player_actions;

#[derive(Debug)]
pub struct Game {
    pub adventuring_parties: HashMap<u32, AdventuringParty>,
    pub time_started: Instant,
    pub id_generator: IdGenerator,
}

impl Game {
    pub fn new() -> Game {
        Game {
            adventuring_parties: HashMap::new(),
            time_started: Instant::now(),
            id_generator: IdGenerator::new(),
        }
    }

    pub fn add_adventuring_party(&mut self) {
        let party_id = self.id_generator.get_next_entity_id();
        let new_party = AdventuringParty::new(party_id, &mut self.id_generator);
        self.adventuring_parties.insert(party_id, new_party);
    }

    pub fn process_player_action(
        &mut self,
        party_id: u32,
        player_action_request: PlayerActionRequest,
    ) {
        // validate action
        //   if action requires player active, check if action comes from active player
        // process the action
        // send client(s) updated adventure info
    }
}
