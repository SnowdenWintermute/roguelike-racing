mod generate_next_room;
use crate::app_consts::error_messages;
use crate::app_consts::error_messages::INVALID_ITEM_ID;
use crate::character::Character;
use crate::combatants::CombatantProperties;
use crate::dungeon_rooms::DungeonRoom;
use crate::dungeon_rooms::DungeonRoomTypes;
use crate::errors::AppError;
use crate::errors::AppErrorTypes;
use crate::items::Item;
use crate::primatives::EntityProperties;
use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RoomsExplored {
    pub total: u16,
    pub on_current_floor: u16,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AdventuringParty {
    pub id: u32,
    pub name: String,
    pub websocket_channel_name: String,
    pub player_usernames: HashSet<String>,
    pub players_ready_to_explore: HashSet<String>,
    pub characters: HashMap<u32, Character>,
    pub character_positions: Vec<u32>,
    pub current_floor: u8,
    pub rooms_explored: RoomsExplored,
    pub current_room: DungeonRoom,
    pub battle_id: Option<u32>,
    pub time_of_wipe: Option<u64>,
    pub time_of_escape: Option<u64>,
    pub items_on_ground_not_yet_received_by_all_clients: HashMap<u32, Vec<u32>>,
}

impl AdventuringParty {
    pub fn new(id: u32, name: String, websocket_channel_name: String) -> AdventuringParty {
        AdventuringParty {
            id,
            name,
            websocket_channel_name,
            player_usernames: HashSet::new(),
            players_ready_to_explore: HashSet::new(),
            characters: HashMap::new(),
            character_positions: Vec::new(),
            current_floor: 1,
            rooms_explored: RoomsExplored {
                total: 1,
                on_current_floor: 1,
            },
            current_room: DungeonRoom {
                room_type: DungeonRoomTypes::Empty,
                treasure_chest: None,
                items: None,
                monsters: None,
            },
            battle_id: None,
            time_of_wipe: None,
            time_of_escape: None,
            items_on_ground_not_yet_received_by_all_clients: HashMap::new(),
        }
    }

    pub fn get_item_by_id<'a>(&'a self, id: u32) -> Option<&'a Item> {
        for (_, character) in &self.characters {
            for (_, item) in &character.combatant_properties.equipment {
                if item.entity_properties.id == id {
                    return Some(item);
                }
            }
            for item in &character.inventory.items {
                if item.entity_properties.id == id {
                    return Some(item);
                }
            }
            if let Some(items) = &self.current_room.items {
                for item in items {
                    if item.entity_properties.id == id {
                        return Some(item);
                    }
                }
            }
            if let Some(monsters) = &self.current_room.monsters {
                for (id, monster) in monsters.iter() {
                    for (_, item) in &monster.combatant_properties.equipment {
                        if item.entity_properties.id == *id {
                            return Some(item);
                        }
                    }
                }
            }
        }

        None
    }

    pub fn remove_character(&mut self, character_id: u32) {
        self.characters.remove(&character_id);
        let mut index_to_remove = None;
        for (index, id) in self.character_positions.iter().enumerate() {
            if *id == character_id {
                index_to_remove = Some(index);
            }
        }
        if let Some(index) = index_to_remove {
            self.character_positions.remove(index);
        }
    }

    pub fn player_owns_character(&self, player_username: &String, character_id: u32) -> bool {
        if let Some(character) = self.characters.get(&character_id) {
            if &character.name_of_controlling_user == player_username {
                return true;
            }
        }
        false
    }

    pub fn get_monster_ids(&self) -> Result<Vec<u32>, AppError> {
        let monsters = self
            .current_room
            .monsters
            .as_ref()
            .ok_or_else(|| AppError {
                error_type: crate::errors::AppErrorTypes::ClientError,
                message: error_messages::ENEMY_COMBATANTS_NOT_FOUND.to_string(),
            })?;
        Ok(monsters.iter().map(|(id, _)| *id).collect::<Vec<u32>>())
    }

    pub fn get_mut_character_if_owned<'a>(
        &'a mut self,
        player_character_ids_option: Option<HashSet<u32>>,
        character_id: u32,
    ) -> Result<&'a mut Character, AppError> {
        let player_character_ids = player_character_ids_option.ok_or_else(|| AppError {
            error_type: crate::errors::AppErrorTypes::ServerError,
            message: error_messages::PLAYER_HAS_NO_CHARACTERS.to_string(),
        })?;
        match player_character_ids.contains(&character_id) {
            true => self
                .characters
                .get_mut(&character_id)
                .ok_or_else(|| AppError {
                    error_type: AppErrorTypes::ServerError,
                    message: error_messages::CHARACTER_NOT_FOUND.to_string(),
                }),
            false => Err(AppError {
                error_type: AppErrorTypes::ServerError,
                message: error_messages::CHARACTER_NOT_OWNED.to_string(),
            }),
        }
    }

    pub fn get_character_if_owned<'a>(
        &'a self,
        player_character_ids_option: Option<HashSet<u32>>,
        character_id: u32,
    ) -> Result<&'a Character, AppError> {
        let player_character_ids = player_character_ids_option.ok_or_else(|| AppError {
            error_type: crate::errors::AppErrorTypes::ServerError,
            message: error_messages::PLAYER_HAS_NO_CHARACTERS.to_string(),
        })?;
        match player_character_ids.contains(&character_id) {
            true => self.characters.get(&character_id).ok_or_else(|| AppError {
                error_type: AppErrorTypes::ServerError,
                message: error_messages::CHARACTER_NOT_FOUND.to_string(),
            }),
            false => Err(AppError {
                error_type: AppErrorTypes::ServerError,
                message: error_messages::CHARACTER_NOT_OWNED.to_string(),
            }),
        }
    }

    pub fn get_combatant_by_id(
        &self,
        id: &u32,
    ) -> Result<(&EntityProperties, &CombatantProperties), AppError> {
        let mut to_return = Err(AppError {
            error_type: AppErrorTypes::ServerError,
            message: error_messages::COMBATANT_NOT_FOUND.to_string(),
        });

        if let Some(character) = self.characters.get(&id) {
            to_return = Ok((
                &character.entity_properties,
                &character.combatant_properties,
            ));
        } else if let Some(monsters) = &self.current_room.monsters {
            if let Some(monster) = &monsters.get(&id) {
                to_return = Ok((&monster.entity_properties, &monster.combatant_properties));
            }
        }
        to_return
    }

    pub fn get_mut_combatant_by_id(
        &mut self,
        id: &u32,
    ) -> Result<(&mut EntityProperties, &mut CombatantProperties), AppError> {
        let mut to_return = Err(AppError {
            error_type: AppErrorTypes::ServerError,
            message: error_messages::COMBATANT_NOT_FOUND.to_string(),
        });

        if let Some(character) = self.characters.get_mut(&id) {
            to_return = Ok((
                &mut character.entity_properties,
                &mut character.combatant_properties,
            ));
        } else if let Some(monsters) = &mut self.current_room.monsters {
            if let Some(monster) = monsters.get_mut(&id) {
                to_return = Ok((
                    &mut monster.entity_properties,
                    &mut monster.combatant_properties,
                ));
            }
        }
        to_return
    }

    pub fn remove_item_from_ground(&mut self, item_id: u32) -> Result<Item, AppError> {
        let item_option = if let Some(items_on_ground) = &mut self.current_room.items {
            let removed_item = Item::remove_item_from_vec(items_on_ground, item_id)?;
            Some(removed_item)
        } else {
            None
        };
        item_option.ok_or_else(|| AppError {
            error_type: AppErrorTypes::InvalidInput,
            message: INVALID_ITEM_ID.to_string(),
        })
    }
}
