use crate::app_consts::error_messages::{self, PLAYER_NOT_FOUND};
use crate::app_consts::DEEPEST_FLOOR;
use crate::character::Character;
use crate::game::id_generator::IdGenerator;
use crate::items::Item;
use crate::{adventuring_party::AdventuringParty, errors::AppError};
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};
use std::{collections::HashMap, collections::HashSet, hash::Hash, time::Instant};

use self::getters::{get_mut_party, get_mut_player};
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
        let mut game = RoguelikeRacerGame {
            name: game_name,
            password: None,
            players: HashMap::new(),
            players_readied: HashSet::new(),
            adventuring_parties: HashMap::new(),
            time_started: None,
            id_generator: IdGenerator::new(),
        };

        for i in 1..=DEEPEST_FLOOR {
            for _ in 0..5 {
                let level = DEEPEST_FLOOR;
                let item = Item::generate(&mut game.id_generator, level);
                match item.item_properties {
                    crate::items::ItemProperties::Consumable(_) => (),
                    crate::items::ItemProperties::Equipment(equipment_properties) => {
                        println!("level {}:  {}", level, equipment_properties.equipment_type);
                        // if let Some(base_ac) = equipment_properties.base_ac {
                        //     println!("Base AC: {}", base_ac)
                        // }
                        // if let Some(base_damage) = equipment_properties.base_damage {
                        //     println!("Base Damage: {:?}", base_damage)
                        // }
                        if let Some(durability) = equipment_properties.durability {
                            println!("Durability: {}/{}", durability.current, durability.max)
                        }
                        for affix in equipment_properties.affixes {
                            match affix {
                                crate::items::affixes::Affix::Prefix(prefix_type, tier) => {
                                    println!("Prefix: {:?} Tier: {}", prefix_type, tier)
                                }
                                crate::items::affixes::Affix::Suffix(suffix_type, tier) => {
                                    println!("Suffix: {:?} Tier: {}", suffix_type, tier)
                                }
                            }
                        }
                        if equipment_properties.attributes.len() > 0 {
                            for (attribute, value) in equipment_properties.attributes {
                                println!("{:?}:{}", attribute, value)
                            }
                        }
                        if equipment_properties.requirements.len() > 0 {
                            println!("Requirements: ");
                            for (attribute, value) in equipment_properties.requirements {
                                println!("{:?}:{}", attribute, value)
                            }
                        }
                        if let Some(traits) = equipment_properties.traits {
                            for item in traits {
                                println!("{:?}", item);
                            }
                        }
                        println!("")
                        // println!(
                        //     "level {} generated {:?} {:?}",
                        //     i, equipment_properties.affixes, equipment_properties.attributes
                        // );
                    }
                }
            }
        }

        game
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
        let party = get_mut_party(self, party_id)?;
        party.player_usernames.insert(username.clone());

        let player_to_move = get_mut_player(self, username)?;
        player_to_move.party_id = Some(party_id);

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
        let player = get_mut_player(self, username)?;
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
}
