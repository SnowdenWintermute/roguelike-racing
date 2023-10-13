use crate::app_consts::error_messages::{self, PLAYER_NOT_FOUND};
use crate::game::id_generator::IdGenerator;
use crate::{adventuring_party::AdventuringParty, errors::AppError};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, hash::Hash, time::Instant};

use self::getters::get_mut_player;
pub mod getters;
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
    pub players: HashMap<String, RoguelikeRacerPlayer>,
    pub adventuring_parties: HashMap<u32, AdventuringParty>,
    pub time_started: Option<u64>,
    pub id_generator: IdGenerator,
}

impl RoguelikeRacerGame {
    pub fn new(game_name: String) -> RoguelikeRacerGame {
        RoguelikeRacerGame {
            name: game_name,
            password: None,
            players: HashMap::new(),
            adventuring_parties: HashMap::new(),
            time_started: None,
            id_generator: IdGenerator::new(),
        }
    }

    pub fn get_number_of_players(&self) -> u8 {
        let mut number_of_players = self.players.len();
        number_of_players as u8
    }

    pub fn add_adventuring_party(&mut self, name: String, id: u32) {
        let new_party = AdventuringParty::new(id, name);
        self.adventuring_parties.insert(id, new_party);
    }

    pub fn put_player_in_adventuring_party(
        &mut self,
        party_id: u32,
        username: String,
    ) -> Result<(), AppError> {
        let party = self
            .adventuring_parties
            .get_mut(&party_id)
            .ok_or(AppError {
                error_type: crate::errors::AppErrorTypes::ServerError,
                message: error_messages::PARTY_NOT_FOUND.to_string(),
            })?;

        let mut player_to_move = self.players.get_mut(&username).ok_or(AppError {
            error_type: crate::errors::AppErrorTypes::ServerError,
            message: error_messages::PLAYER_NOT_FOUND.to_string(),
        })?;

        player_to_move.party_id = Some(party_id);
        party.player_usernames.insert(username);
        Ok(())
    }

    pub fn remove_player_from_adventuring_party(
        &mut self,
        username: String,
    ) -> Result<(), AppError> {
        let player = get_mut_player(self, username.clone())?;
        if player.party_id.is_none() {
            return Ok(());
        }
        let party_id_leaving = player.party_id;
        player.character_ids = None;
        player.party_id = None;
        let character_ids = player.character_ids.clone();

        let mut party = self
            .adventuring_parties
            .get_mut(&party_id_leaving.expect("none check just above here"))
            .ok_or(AppError {
                error_type: crate::errors::AppErrorTypes::ServerError,
                message: error_messages::PARTY_NOT_FOUND.to_string(),
            })?;

        match &character_ids {
            Some(character_ids) => {
                for character_id in character_ids {
                    party.player_characters.remove(&character_id);
                }
            }
            _ => (),
        };

        party.player_usernames.remove(&username);

        // clean up empty parties
        let mut party_ids_to_remove = Vec::new();
        for (id, party) in &self.adventuring_parties {
            if party.player_usernames.len() < 1 {
                party_ids_to_remove.push(*id);
            }
        }

        for id in party_ids_to_remove {
            self.adventuring_parties.remove(&id);
        }
        Ok(())
    }
}
