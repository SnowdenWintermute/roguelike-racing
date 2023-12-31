use super::BattleGroup;
use crate::{
    combatants::{combat_attributes::CombatAttributes, CombatantProperties},
    errors::AppError,
    game::{getters::get_party, RoguelikeRacerGame},
};
use serde::{Deserialize, Serialize};

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

        // when a combatant takes their turn, subtract the average speed of the group representing the time it
        // takes to do an action from their movement
        // sort combatants by movement again and repeat until the first combatant in line has less
        // movement than a turn requires TURN_TIME
        // refill every combatant's movement by their speed (their movement might be negative)
    }
}
