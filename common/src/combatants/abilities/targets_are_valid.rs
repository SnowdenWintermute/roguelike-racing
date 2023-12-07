use super::{
    get_combatant_ability_attributes::{TargetCategories, TargetingScheme},
    CombatantAbility,
};
use crate::adventuring_party::AdventuringParty;

impl CombatantAbility {
    pub fn targets_are_valid(
        &self,
        target_ids_option: &Option<Vec<u32>>,
        party: &AdventuringParty,
    ) -> bool {
        let ability_attributes = self.ability_name.get_attributes();
        match target_ids_option {
            Some(target_ids) => match self.selected_targeting_scheme {
                TargetingScheme::Single => {
                    if target_ids.len() > 1 {
                        return false;
                    } else {
                        return match ability_attributes.valid_target_categories {
                            TargetCategories::Opponent => {
                                is_id_of_existing_opponent(&party, &target_ids[0])
                            }
                            // if the character is using this
                            // ability, they still exist and may
                            // target themself
                            TargetCategories::User => true,
                            TargetCategories::Friendly => {
                                party.characters.get(&target_ids[0]).is_some()
                            }
                            TargetCategories::Any => {
                                party.characters.get(&target_ids[0]).is_some()
                                    || is_id_of_existing_opponent(&party, &target_ids[0])
                            }
                        };
                    }
                }
                TargetingScheme::Area => match ability_attributes.valid_target_categories {
                    TargetCategories::Opponent => {
                        let number_of_monsters = match &party.current_room.monsters {
                            Some(monsters) => monsters.len(),
                            None => 0,
                        };

                        if number_of_monsters != target_ids.len() {
                            false
                        } else {
                            let mut to_return = true;
                            for id in target_ids.iter() {
                                if !is_id_of_existing_opponent(&party, id) {
                                    to_return = false;
                                    break;
                                }
                            }
                            to_return
                        }
                    }
                    TargetCategories::User => true,
                    TargetCategories::Friendly => {
                        if party.characters.len() != target_ids.len() {
                            false
                        } else {
                            let mut to_return = true;
                            for id in target_ids {
                                if !&party.characters.get(&id).is_some() {
                                    to_return = false;
                                    break;
                                }
                            }
                            to_return
                        }
                    }
                    TargetCategories::Any => {
                        let mut to_return = true;
                        for id in target_ids {
                            if !party.characters.get(&id).is_some()
                                && !is_id_of_existing_opponent(&party, &id)
                            {
                                to_return = false;
                                break;
                            }
                        }
                        to_return
                    }
                },
            },

            None => return false,
        }
    }
}

fn is_id_of_existing_opponent(party: &AdventuringParty, id: &u32) -> bool {
    match &party.current_room.monsters {
        Some(monsters) => monsters
            .iter()
            .map(|monster| monster.entity_properties.id)
            .collect::<Vec<u32>>()
            .contains(id),
        None => false,
    }
}
