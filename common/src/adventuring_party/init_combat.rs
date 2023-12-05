use super::AdventuringParty;
use crate::combatants::{CombatAttributes, CombatantProperties};
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

impl AdventuringParty {
    pub fn get_combat_turn_order(&mut self) -> Vec<CombatantTurnTracker> {
        let mut combatant_turn_trackers = vec![];

        // get "speed" value for each combatant
        // give each combatant "movement" equal to their "speed"
        for id in self.character_positions.iter() {
            let character = self
                .characters
                .get(id)
                .expect("to have a valid character ref");
            let turn_tracker =
                get_turn_tracker_from_combatant(&character.combatant_properties, *id);
            combatant_turn_trackers.push(turn_tracker);
        }

        for monster in self
            .current_room
            .monsters
            .as_ref()
            .expect("to be called when monsters are present")
            .iter()
        {
            let turn_tracker = get_turn_tracker_from_combatant(
                &monster.combatant_properties,
                monster.entity_properties.id,
            );
            combatant_turn_trackers.push(turn_tracker);
        }
        // set the highest movement combatant as the active combatant
        combatant_turn_trackers.sort_by(|a, b| b.movement.partial_cmp(&a.movement).unwrap());
        println!("trackers: {:#?}", combatant_turn_trackers);

        combatant_turn_trackers
        // when a combatant takes their turn, subtract the average speed of the group representing the time it
        // takes to do an action from their movement
        // sort combatants by movement again and repeat until the first combatant in line has less
        // movement than a turn requires TURN_TIME
        // refill every combatant's movement by their speed (their movement might be negative)
    }
}
