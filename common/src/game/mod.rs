use serde::{Deserialize, Serialize};

use crate::adventuring_party::AdventuringParty;
use crate::game::id_generator::IdGenerator;
use std::{collections::HashMap, hash::Hash, time::Instant};
pub mod id_generator;
pub mod player_actions;
pub mod player_input_handlers;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RoguelikeRacerPlayer {
    pub actor_id: Option<u32>,
    pub party_id: Option<u32>,
    pub username: String,
    pub character_ids: Option<Vec<u32>>,
    pub ready: bool,
}

impl RoguelikeRacerPlayer {
    pub fn new(actor_id: Option<u32>, username: String) -> Self {
        RoguelikeRacerPlayer {
            actor_id,
            party_id: None,
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
            println!(
                "counting players in party {}, number: {}",
                party.name.clone(),
                party.players.len()
            );
            number_of_players += party.players.len();
        }
        println!(
            "{} players in game {}",
            number_of_players,
            self.name.clone()
        );
        number_of_players as u8
    }

    pub fn add_adventuring_party(&mut self, name: String) -> u32 {
        let party_id = self.id_generator.get_next_entity_id();
        let new_party = AdventuringParty::new(party_id, name);
        self.adventuring_parties.insert(party_id, new_party);
        party_id
    }

    pub fn put_player_in_adventuring_party(&mut self, party_id: u32, username: String) {
        // add them to the party
        if let Some(party) = self.adventuring_parties.get_mut(&party_id) {
            // remove them from partyless players list
            if let Some(mut player_to_move) = self.partyless_players.remove(&username) {
                player_to_move.party_id = Some(party_id);
                party.players.insert(username, player_to_move);
            } else {
                println!("tried to put a player into party id {} but they weren't found in the list of partyless players", party_id);
            }
        } else {
            println!(
                "tried to put {} into party id {} but the party wasn't found in the current game",
                &username, party_id
            );
        }
    }

    pub fn remove_player_from_adventuring_party(
        &mut self,
        username: String,
        put_in_partyless_players: bool,
    ) {
        for (id, party) in self.adventuring_parties.iter_mut() {
            let player_removed_option = party.remove_player_and_their_characters(username.clone());
            if (put_in_partyless_players) {
                match player_removed_option {
                    Some(player) => {
                        self.partyless_players
                            .insert(player.username.clone(), player);
                    }
                    None => (),
                }
            }
        }

        let mut party_ids_to_remove = Vec::new();
        for (id, party) in &self.adventuring_parties {
            if party.players.len() < 1 {
                party_ids_to_remove.push(*id);
            }
        }

        for id in party_ids_to_remove {
            self.adventuring_parties.remove(&id);
        }
    }
}
