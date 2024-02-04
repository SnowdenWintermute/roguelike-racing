use super::turn_order::CombatantTurnTracker;
use crate::app_consts::error_messages;
use crate::combatants::CombatantProperties;
use crate::errors::AppError;
use crate::errors::AppErrorTypes;
use crate::game::RoguelikeRacerGame;
use crate::primatives::EntityProperties;
use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct Battle {
    pub id: u32,
    pub group_a: BattleGroup,
    pub group_b: BattleGroup,
    pub combatant_turn_trackers: Vec<CombatantTurnTracker>,
}

impl Battle {
    pub fn combatant_is_first_in_turn_order(&self, entity_id: u32) -> bool {
        let first_turn_tracker_option = self.combatant_turn_trackers.first();
        if let Some(first_turn_tracker) = first_turn_tracker_option {
            return first_turn_tracker.entity_id == entity_id;
        } else {
            return false;
        }
    }

    pub fn get_ally_ids_and_opponent_ids_option(
        &self,
        combatant_id: u32,
    ) -> Result<(Vec<u32>, Option<Vec<u32>>), AppError> {
        let opponent_ids_option = if self.group_a.combatant_ids.contains(&combatant_id) {
            Some(self.group_b.combatant_ids.clone())
        } else if self.group_b.combatant_ids.contains(&combatant_id) {
            Some(self.group_a.combatant_ids.clone())
        } else {
            None
        };

        let ally_ids = if self.group_a.combatant_ids.contains(&combatant_id) {
            Some(self.group_a.combatant_ids.clone())
        } else if self.group_b.combatant_ids.contains(&combatant_id) {
            Some(self.group_b.combatant_ids.clone())
        } else {
            None
        }
        .ok_or_else(|| AppError {
            error_type: crate::errors::AppErrorTypes::Generic,
            message: error_messages::ALLY_COMBATANTS_NOT_FOUND.to_string(),
        })?;

        Ok((ally_ids, opponent_ids_option))
    }

    pub fn get_all_combatant_ids(&self) -> Vec<u32> {
        let mut to_return = vec![];
        for id in &self.group_a.combatant_ids {
            to_return.push(*id)
        }
        for id in &self.group_b.combatant_ids {
            to_return.push(*id)
        }
        to_return
    }

    pub fn get_ally_and_enemy_battle_groups(
        &self,
        combatant_id: &u32,
    ) -> Result<(BattleGroup, BattleGroup), AppError> {
        for id in &self.group_a.combatant_ids {
            if id == combatant_id {
                return Ok((self.group_a.clone(), self.group_b.clone()));
            }
        }
        for id in &self.group_b.combatant_ids {
            if id == combatant_id {
                return Ok((self.group_b.clone(), self.group_a.clone()));
            }
        }
        return Err(AppError {
            error_type: AppErrorTypes::Generic,
            message: error_messages::COMBATANT_BATTLE_MISMATCH.to_string(),
        });
    }

    pub fn is_id_of_existing_opponent(&self, combatant_id: u32, target_id: u32) -> bool {
        let mut to_return = false;
        if let Ok((_, opponent_ids_option)) =
            self.get_ally_ids_and_opponent_ids_option(combatant_id)
        {
            if let Some(opponent_ids) = opponent_ids_option {
                to_return = opponent_ids.contains(&target_id)
            }
        }
        to_return
    }
}

/// creates a new battle and returns the battle_id
impl RoguelikeRacerGame {
    pub fn initiate_battle(
        &mut self,
        group_a: BattleGroup,
        group_b: BattleGroup,
    ) -> Result<u32, AppError> {
        let turn_trackers = self.create_turn_trackers(&group_a, &group_b)?;
        let battle = Battle {
            id: self.id_generator.get_next_entity_id(),
            group_a,
            group_b,
            combatant_turn_trackers: turn_trackers,
        };
        let battle_id = battle.id;

        self.battles.insert(battle_id, battle);
        Ok(battle_id)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub enum BattleGroupTypes {
    #[default]
    PlayerControlled,
    ComputerControlled,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct BattleGroup {
    pub name: String,
    pub party_id: u32,
    pub combatant_ids: Vec<u32>,
    pub group_type: BattleGroupTypes,
}

impl BattleGroup {
    pub fn get_battle_group_entity_belongs_to(
        ally_battle_group: BattleGroup,
        opponent_battle_group_option: Option<BattleGroup>,
        id: &u32,
    ) -> Result<BattleGroup, AppError> {
        if ally_battle_group.combatant_ids.contains(id) {
            Ok(ally_battle_group)
        } else if let Some(opponent_battle_group) = opponent_battle_group_option {
            if opponent_battle_group.combatant_ids.contains(id) {
                Ok(opponent_battle_group)
            } else {
                Err(AppError {
                    error_type: AppErrorTypes::Generic,
                    message: error_messages::COMBATANT_NOT_FOUND.to_string(),
                })
            }
        } else {
            Err(AppError {
                error_type: AppErrorTypes::Generic,
                message: error_messages::COMBATANT_NOT_FOUND.to_string(),
            })
        }
    }
}

impl RoguelikeRacerGame {
    pub fn get_combatant_in_battle_by_id(
        &self,
        battle: &Battle,
        combatant_id: &u32,
    ) -> Result<(&EntityProperties, &CombatantProperties), AppError> {
        let party_id = if battle.group_a.combatant_ids.contains(&combatant_id) {
            battle.group_a.party_id
        } else if battle.group_b.combatant_ids.contains(&combatant_id) {
            battle.group_b.party_id
        } else {
            return Err(AppError {
                error_type: AppErrorTypes::Generic,
                message: error_messages::COMBATANT_NOT_FOUND.to_string(),
            });
        };

        let party = self
            .adventuring_parties
            .get(&party_id)
            .ok_or_else(|| AppError {
                error_type: AppErrorTypes::Generic,
                message: error_messages::PARTY_NOT_FOUND.to_string(),
            })?;

        party.get_combatant_by_id(combatant_id)
    }

    pub fn get_mut_combatant_in_battle_by_id(
        &mut self,
        battle: &Battle,
        combatant_id: &u32,
    ) -> Result<(&mut EntityProperties, &mut CombatantProperties), AppError> {
        let party_id = if battle.group_a.combatant_ids.contains(&combatant_id) {
            battle.group_a.party_id
        } else if battle.group_b.combatant_ids.contains(&combatant_id) {
            battle.group_b.party_id
        } else {
            return Err(AppError {
                error_type: AppErrorTypes::Generic,
                message: error_messages::COMBATANT_NOT_FOUND.to_string(),
            });
        };

        let party = self
            .adventuring_parties
            .get_mut(&party_id)
            .ok_or_else(|| AppError {
                error_type: AppErrorTypes::Generic,
                message: error_messages::PARTY_NOT_FOUND.to_string(),
            })?;

        party.get_mut_combatant_by_id(combatant_id)
    }

    pub fn get_mut_combatant_in_battle_group_by_id(
        &mut self,
        battle_group: &BattleGroup,
        id: u32,
    ) -> Result<(&mut EntityProperties, &mut CombatantProperties), AppError> {
        let party_option = self.adventuring_parties.get_mut(&battle_group.party_id);
        if let Some(party) = party_option {
            party.get_mut_combatant_by_id(&id)
        } else {
            return Err(AppError {
                error_type: AppErrorTypes::ServerError,
                message: error_messages::PARTY_NOT_FOUND.to_string(),
            });
        }
    }
}
