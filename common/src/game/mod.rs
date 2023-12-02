use self::getters::get_mut_party;
use self::getters::get_mut_player;
use crate::adventuring_party::AdventuringParty;
use crate::character::Character;
use crate::errors::AppError;
use crate::game::id_generator::IdGenerator;
use crate::items::Item;
use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;
use std::collections::HashSet;
use std::time::SystemTime;
use std::time::UNIX_EPOCH;
pub mod add_character_to_adventuring_party;
pub mod getters;
pub mod id_generator;
pub mod player_actions;
pub mod player_input_handlers;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RoguelikeRacerPlayer {
    pub actor_id: Option<u32>,
    pub party_id: Option<u32>,
    pub username: String,
    pub character_ids: Option<HashSet<u32>>,
}

impl RoguelikeRacerPlayer {
    pub fn new(actor_id: Option<u32>, username: String) -> Self {
        RoguelikeRacerPlayer {
            actor_id,
            party_id: None,
            username,
            character_ids: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct RoguelikeRacerGame {
    pub name: String,
    pub password: Option<String>,
    pub players: HashMap<String, RoguelikeRacerPlayer>,
    pub players_readied: HashSet<String>,
    pub adventuring_parties: HashMap<u32, AdventuringParty>,
    pub time_started: Option<u128>,
    pub id_generator: IdGenerator,
}

impl RoguelikeRacerGame {
    pub fn new(game_name: String) -> RoguelikeRacerGame {
        let game = RoguelikeRacerGame {
            name: game_name,
            password: None,
            players: HashMap::new(),
            players_readied: HashSet::new(),
            adventuring_parties: HashMap::new(),
            time_started: None,
            id_generator: IdGenerator::new(),
        };

        // print_random_equipments(&mut game);
        game
    }

    pub fn get_number_of_players(&self) -> u8 {
        let number_of_players = self.players.len();
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
        let party = get_mut_party(self, party_id)?;
        party.player_usernames.insert(username.clone());

        let player_to_move = get_mut_player(self, &username)?;
        player_to_move.party_id = Some(party_id);

        Ok(())
    }

    pub fn remove_player_from_adventuring_party(
        &mut self,
        username: String,
    ) -> Result<(), AppError> {
        let player = get_mut_player(self, &username)?;
        if player.party_id.is_none() {
            return Ok(());
        }
        let party_id_leaving = player.party_id;
        let character_ids = player.character_ids.clone();
        player.character_ids = None;
        player.party_id = None;

        let party = get_mut_party(self, party_id_leaving.expect("none check just above here"))?;

        match &character_ids {
            Some(character_ids) => {
                for character_id in character_ids {
                    party.characters.remove(&character_id);
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

    pub fn get_player_characters(
        &mut self,
        username: String,
    ) -> Result<HashMap<u32, Character>, AppError> {
        let player = get_mut_player(self, &username)?;
        let party_id_option = player.party_id;
        let character_ids = player.character_ids.clone();
        let mut characters = HashMap::new();
        if let Some(party_id) = party_id_option {
            let party = get_mut_party(self, party_id)?;
            let character_ids = match character_ids {
                Some(ids) => ids,
                None => HashSet::new(),
            };
            for id in character_ids {
                if let Some(character) = party.characters.get(&id) {
                    characters.insert(id, character.clone());
                }
            }
        }
        Ok(characters)
    }

    pub fn start(&mut self) {
        self.time_started = Some(
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("time went backwards")
                .as_millis(),
        );

        // for party in &self.adventuring_parties {

        // }
    }

    pub fn get_item_in_adventuring_party(
        &self,
        adventuring_party_id: u32,
        item_id: u32,
    ) -> Option<&Item> {
        let adventuring_party_option = self.adventuring_parties.get(&adventuring_party_id);
        if let Some(party) = adventuring_party_option {
            return party.get_item_by_id(item_id);
        }
        None
    }
}
