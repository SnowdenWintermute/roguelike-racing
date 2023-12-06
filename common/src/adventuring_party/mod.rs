mod generate_next_room;
pub mod init_combat;
use self::init_combat::CombatantTurnTracker;
use crate::character::Character;
use crate::dungeon_rooms::DungeonRoom;
use crate::dungeon_rooms::DungeonRoomTypes;
use crate::items::Item;
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
    pub player_usernames: HashSet<String>,
    pub players_ready_to_explore: HashSet<String>,
    pub characters: HashMap<u32, Character>,
    pub character_positions: Vec<u32>,
    pub combatant_turn_trackers: Option<Vec<CombatantTurnTracker>>,
    pub current_floor: u8,
    pub rooms_explored: RoomsExplored,
    pub current_room: DungeonRoom,
    pub time_of_wipe: Option<u64>,
    pub time_of_escape: Option<u64>,
}

impl AdventuringParty {
    pub fn new(id: u32, name: String) -> AdventuringParty {
        AdventuringParty {
            id,
            name,
            player_usernames: HashSet::new(),
            players_ready_to_explore: HashSet::new(),
            characters: HashMap::new(),
            character_positions: Vec::new(),
            combatant_turn_trackers: None,
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
            time_of_wipe: None,
            time_of_escape: None,
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
                for monster in monsters {
                    for (_, item) in &monster.combatant_properties.equipment {
                        if item.entity_properties.id == id {
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

    pub fn player_owns_character(&self, player_username: String, character_id: u32) -> bool {
        if let Some(character) = self.characters.get(&character_id) {
            if character.name_of_controlling_user == player_username {
                return true;
            }
        }
        false
    }

    pub fn combatant_is_first_in_turn_order(&self, entity_id: u32) -> bool {
        match &self.combatant_turn_trackers {
            Some(trackers) => match trackers.get(0) {
                Some(combat_turn_tracker) => combat_turn_tracker.entity_id == entity_id,
                None => false,
            },
            None => false,
        }
    }

    // @TODO - optimize a second function to only get target ids for active combatant
    pub fn get_all_targeted_ids_by_combatant_id(&self) -> Option<HashMap<u32, Vec<u32>>> {
        let mut ability_targets_by_combatant_id = HashMap::new();
        for (id, character) in &self.characters {
            let targets_option = &character.combatant_properties.ability_target_ids;
            if let Some(targets) = targets_option {
                ability_targets_by_combatant_id.insert(*id, targets.clone());
            }
        }
        if let Some(monsters) = &self.current_room.monsters {
            for monster in monsters {
                let targets_option = &monster.combatant_properties.ability_target_ids;
                if let Some(targets) = targets_option {
                    ability_targets_by_combatant_id
                        .insert(monster.entity_properties.id, targets.clone());
                }
            }
        }

        if ability_targets_by_combatant_id.len() > 0 {
            Some(ability_targets_by_combatant_id)
        } else {
            None
        }
    }
}
