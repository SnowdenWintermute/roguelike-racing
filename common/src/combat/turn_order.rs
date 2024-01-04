use super::BattleGroup;
use crate::app_consts::error_messages;
use crate::app_consts::TURN_TIME;
use crate::combatants::combat_attributes::CombatAttributes;
use crate::combatants::CombatantProperties;
use crate::errors::AppError;
use crate::game::getters::get_party;
use crate::game::RoguelikeRacerGame;
use serde::Deserialize;
use serde::Serialize;

#[derive(PartialEq, Eq, Serialize, Deserialize, Clone, Debug)]
pub struct CombatantTurnTracker {
    pub entity_id: u32,
    pub movement: i16,
}

pub fn get_turn_tracker_from_combatant(
    combatant_properties: &CombatantProperties,
    id: u32,
) -> CombatantTurnTracker {
    let speed = combatant_properties
        .get_total_attributes()
        .get(&CombatAttributes::Speed)
        .unwrap_or_else(|| &0)
        .clone();

    CombatantTurnTracker {
        entity_id: id,
        movement: speed as i16,
    }
}

impl RoguelikeRacerGame {
    pub fn get_battle_turn_order(
        &self,
        group_a: &BattleGroup,
        group_b: &BattleGroup,
    ) -> Result<Vec<CombatantTurnTracker>, AppError> {
        let mut combatant_turn_trackers = vec![];

        let party_a = get_party(self, group_a.party_id)?;
        for entity_id in &group_a.combatant_ids {
            let (_, combatant_properties) = party_a.get_combatant_by_id(entity_id)?;
            let turn_tracker = get_turn_tracker_from_combatant(&combatant_properties, *entity_id);
            combatant_turn_trackers.push(turn_tracker);
        }

        let party_b = get_party(self, group_b.party_id)?;
        for entity_id in &group_b.combatant_ids {
            let (_, combatant_properties) = party_b.get_combatant_by_id(entity_id)?;
            let turn_tracker = get_turn_tracker_from_combatant(&combatant_properties, *entity_id);
            combatant_turn_trackers.push(turn_tracker);
        }

        combatant_turn_trackers.sort_by(|a, b| b.movement.partial_cmp(&a.movement).unwrap());

        Ok(combatant_turn_trackers)
    }

    pub fn end_active_combatant_turn(
        &mut self,
        battle_id: u32,
    ) -> Result<&CombatantTurnTracker, AppError> {
        let battle = self.battles.get_mut(&battle_id).ok_or_else(|| AppError {
            error_type: crate::errors::AppErrorTypes::Generic,
            message: error_messages::BATTLE_NOT_FOUND.to_string(),
        })?;
        let mut turn_trackers = battle.combatant_turn_trackers.clone();
        let active_combatant_turn_tracker = turn_trackers.first_mut().ok_or_else(|| AppError {
            error_type: crate::errors::AppErrorTypes::Generic,
            message: error_messages::TURN_TRACKERS_EMPTY.to_string(),
        })?;

        active_combatant_turn_tracker.movement -= TURN_TIME;
        turn_trackers.sort_by(|a, b| b.movement.partial_cmp(&a.movement).unwrap());

        let mut active_combatant_turn_tracker =
            turn_trackers.first_mut().ok_or_else(|| AppError {
                error_type: crate::errors::AppErrorTypes::Generic,
                message: error_messages::TURN_TRACKERS_EMPTY.to_string(),
            })?;

        while active_combatant_turn_tracker.movement < TURN_TIME {
            // tick the time and refill all movement
            for tracker in &mut turn_trackers {
                let (_, combatant_properties) = self.get_combatant_by_id(&tracker.entity_id)?;
                let entity_speed = *combatant_properties
                    .get_total_attributes()
                    .get(&CombatAttributes::Speed)
                    .unwrap_or_else(|| &0) as i16;
                tracker.movement += entity_speed;
            }

            turn_trackers.sort_by(|a, b| b.movement.partial_cmp(&a.movement).unwrap());
            active_combatant_turn_tracker = turn_trackers.first_mut().ok_or_else(|| AppError {
                error_type: crate::errors::AppErrorTypes::Generic,
                message: error_messages::TURN_TRACKERS_EMPTY.to_string(),
            })?;
        }

        let battle = self.battles.get_mut(&battle_id).ok_or_else(|| AppError {
            error_type: crate::errors::AppErrorTypes::Generic,
            message: error_messages::BATTLE_NOT_FOUND.to_string(),
        })?;

        battle.combatant_turn_trackers = turn_trackers;
        let new_active_combatant_turn_tracker =
            battle
                .combatant_turn_trackers
                .first()
                .ok_or_else(|| AppError {
                    error_type: crate::errors::AppErrorTypes::Generic,
                    message: error_messages::TURN_TRACKERS_EMPTY.to_string(),
                })?;

        Ok(new_active_combatant_turn_tracker)
    }
}
