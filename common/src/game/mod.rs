use serde::{Deserialize, Serialize};

use crate::adventuring_party::AdventuringParty;
use crate::game::id_generator::IdGenerator;
use std::{collections::HashMap, hash::Hash, time::Instant};
pub mod id_generator;
pub mod player_actions;
pub mod player_input_handlers;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoguelikeRacerPlayer {
    pub actor_id: Option<usize>,
    pub username: String,
    pub character_ids: Option<Vec<u32>>,
    pub ready: bool,
}

impl RoguelikeRacerPlayer {
    pub fn new(actor_id: Option<usize>, username: String) -> Self {
        RoguelikeRacerPlayer {
            actor_id,
            username,
            character_ids: None,
            ready: false,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoguelikeRacerGame {
    pub name: String,
    pub password: Option<String>,
    pub partyless_players: HashMap<String, RoguelikeRacerPlayer>,
    pub adventuring_parties: HashMap<u32, AdventuringParty>,
    pub time_started: Option<u64>,
    pub id_generator: IdGenerator,
}

impl RoguelikeRacerGame {
    pub fn new(game_name: String) -> RoguelikeRacerGame {
        RoguelikeRacerGame {
            name: game_name,
            password: None,
            partyless_players: HashMap::new(),
            adventuring_parties: HashMap::new(),
            time_started: None,
            id_generator: IdGenerator::new(),
        }
    }

    pub fn get_number_of_players(&self) -> u8 {
        let mut number_of_players = self.partyless_players.len();
        for (_, party) in self.adventuring_parties.iter() {
            number_of_players += party.players.len();
        }
        number_of_players as u8
    }

    pub fn add_adventuring_party(&mut self) {
        let party_id = self.id_generator.get_next_entity_id();
        let new_party = AdventuringParty::new(party_id);
        self.adventuring_parties.insert(party_id, new_party);
    }
}
