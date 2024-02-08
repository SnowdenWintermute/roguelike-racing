use super::BattleGroup;
use crate::app_consts::error_messages;
use crate::combatants::combat_attributes::CombatAttributes;
use crate::errors::AppError;
use crate::game::RoguelikeRacerGame;
use serde::Deserialize;
use serde::Serialize;
use std::cmp::Reverse;

const SPEED_MODIFIER: u32 = 10;
const REQUIRED_MOVEMENT_TO_MOVE: u32 = 999;
const MAX_TICKS_TO_FILL_MOVEMENT: u32 = 10;
const MIN_TICKS_TO_FILL_MOVEMENT: u32 = 5;
const MIN_MOVEMENT_PER_TICK: u32 = REQUIRED_MOVEMENT_TO_MOVE / MAX_TICKS_TO_FILL_MOVEMENT;
const MAX_MOVEMENT_PER_TICK: u32 = REQUIRED_MOVEMENT_TO_MOVE / MIN_TICKS_TO_FILL_MOVEMENT;
const MOVEMENT_RANGE: u32 = MAX_MOVEMENT_PER_TICK - MIN_MOVEMENT_PER_TICK;
const MAX_SPEED: u32 = MIN_MOVEMENT_PER_TICK * 10;
const MIN_SPEED: u32 = MIN_MOVEMENT_PER_TICK * 10 / REQUIRED_MOVEMENT_TO_MOVE * 10;
const SPEED_RANGE: u32 = MAX_SPEED - MIN_SPEED;

#[derive(PartialEq, Eq, Serialize, Deserialize, Clone, Debug)]
pub struct CombatantTurnTracker {
    pub entity_id: u32,
    pub movement: u16,
}

pub fn get_turn_tracker_from_combatant(id: u32) -> CombatantTurnTracker {
    CombatantTurnTracker {
        entity_id: id,
        movement: 0,
    }
}

impl RoguelikeRacerGame {
    pub fn create_turn_trackers(
        &self,
        group_a: &BattleGroup,
        group_b: &BattleGroup,
    ) -> Result<Vec<CombatantTurnTracker>, AppError> {
        let mut combatant_turn_trackers = vec![];

        for entity_id in &group_a.combatant_ids {
            let turn_tracker = get_turn_tracker_from_combatant(*entity_id);
            combatant_turn_trackers.push(turn_tracker);
        }

        for entity_id in &group_b.combatant_ids {
            let turn_tracker = get_turn_tracker_from_combatant(*entity_id);
            combatant_turn_trackers.push(turn_tracker);
        }

        Ok(combatant_turn_trackers)
    }

    pub fn end_active_combatant_turn(
        &mut self,
        battle_id: u32,
    ) -> Result<&CombatantTurnTracker, AppError> {
        self.tick_combat_until_next_combatant_is_active(battle_id)?;
        let battle = self.battles.get_mut(&battle_id).ok_or_else(|| AppError {
            error_type: crate::errors::AppErrorTypes::Generic,
            message: error_messages::BATTLE_NOT_FOUND.to_string(),
        })?;
        let active_combatant_turn_tracker =
            battle
                .combatant_turn_trackers
                .first_mut()
                .ok_or_else(|| AppError {
                    error_type: crate::errors::AppErrorTypes::Generic,
                    message: error_messages::TURN_TRACKERS_EMPTY.to_string(),
                })?;
        active_combatant_turn_tracker.movement -= REQUIRED_MOVEMENT_TO_MOVE as u16;

        battle
            .combatant_turn_trackers
            .sort_by_key(|item| (Reverse(item.movement), item.entity_id));

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

    pub fn tick_combat_until_next_combatant_is_active(
        &mut self,
        battle_id: u32,
    ) -> Result<(), AppError> {
        let battle = self.battles.get_mut(&battle_id).ok_or_else(|| AppError {
            error_type: crate::errors::AppErrorTypes::Generic,
            message: error_messages::BATTLE_NOT_FOUND.to_string(),
        })?;
        let mut cloned_turn_trackers = battle.combatant_turn_trackers.clone();

        cloned_turn_trackers.sort_by_key(|item| (Reverse(item.movement), item.entity_id));

        let mut active_combatant_turn_tracker =
            cloned_turn_trackers.first_mut().ok_or_else(|| AppError {
                error_type: crate::errors::AppErrorTypes::Generic,
                message: error_messages::TURN_TRACKERS_EMPTY.to_string(),
            })?;

        while active_combatant_turn_tracker.movement < REQUIRED_MOVEMENT_TO_MOVE as u16 {
            // tick the time and refill all movement
            for tracker in &mut cloned_turn_trackers {
                let (_, combatant_properties) = self.get_combatant_by_id(&tracker.entity_id)?;
                let entity_speed = *combatant_properties
                    .get_total_attributes()
                    .get(&CombatAttributes::Speed)
                    .unwrap_or_else(|| &0);
                let adjusted_speed = entity_speed as u32 * SPEED_MODIFIER;
                let movement_to_add = ((((adjusted_speed - MIN_SPEED) * MOVEMENT_RANGE)
                    / SPEED_RANGE)
                    + MIN_MOVEMENT_PER_TICK) as u16;
                tracker.movement += movement_to_add;
            }

            cloned_turn_trackers.sort_by_key(|item| (Reverse(item.movement), item.entity_id));
            active_combatant_turn_tracker =
                cloned_turn_trackers.first_mut().ok_or_else(|| AppError {
                    error_type: crate::errors::AppErrorTypes::Generic,
                    message: error_messages::TURN_TRACKERS_EMPTY.to_string(),
                })?;
        }

        let battle = self.battles.get_mut(&battle_id).ok_or_else(|| AppError {
            error_type: crate::errors::AppErrorTypes::Generic,
            message: error_messages::BATTLE_NOT_FOUND.to_string(),
        })?;

        battle.combatant_turn_trackers = cloned_turn_trackers;
        Ok(())
    }
}
