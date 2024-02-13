use self::getters::get_mut_party;
use self::getters::get_mut_player;
use crate::adventuring_party::AdventuringParty;
use crate::character::Character;
use crate::combat::battle::Battle;
use crate::errors::AppError;
use crate::game::id_generator::IdGenerator;
use crate::game::player::RoguelikeRacerPlayer;
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
pub mod player;
pub mod player_input_handlers;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct RoguelikeRacerGame {
    pub name: String,
    pub password: Option<String>,
    pub players: HashMap<String, RoguelikeRacerPlayer>,
    pub players_readied: HashSet<String>,
    pub adventuring_parties: HashMap<u32, AdventuringParty>,
    pub battles: HashMap<u32, Battle>,
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
            battles: HashMap::new(),
            time_started: None,
            id_generator: IdGenerator::new(),
        };

        game
    }

    pub fn get_party_channel_name(&self, party_id: u32) -> String {
        format!("{}-party-{}", self.name, party_id)
    }

    pub fn get_number_of_players(&self) -> u8 {
        let number_of_players = self.players.len();
        number_of_players as u8
    }

    pub fn add_adventuring_party(&mut self, name: String, id: u32) {
        let new_party = AdventuringParty::new(id, name, self.get_party_channel_name(id));
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
    ) -> Result<Option<u32>, AppError> {
        let player = get_mut_player(self, &username)?;
        if player.party_id.is_none() {
            return Ok(None);
        }
        let party_id_leaving = player.party_id.expect("is_none checked");
        let character_ids = player.character_ids.clone();
        player.character_ids = None;
        player.party_id = None;

        let party = get_mut_party(self, party_id_leaving)?;

        match &character_ids {
            Some(character_ids) => {
                for character_id in character_ids {
                    party.remove_character(*character_id)
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

        Ok(Some(party_id_leaving))
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
