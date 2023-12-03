use super::abilities::CombatantAbility;
use crate::adventuring_party::AdventuringParty;

impl CombatantAbility {
    pub fn last_targets_are_still_valid(&self, party: &AdventuringParty) -> bool {
        let last_targets = &self.most_recently_targeted;
        match last_targets {
            Some(target_ids) => {
                for id in target_ids.iter() {
                    let is_existing_ally = party.character_positions.contains(id);
                    let is_existing_opponent = match &party.current_room.monsters {
                        Some(monsters) => monsters
                            .iter()
                            .map(|monster| monster.entity_properties.id)
                            .collect::<Vec<u32>>()
                            .contains(id),
                        None => false,
                    };
                    if !is_existing_ally && !is_existing_opponent {
                        return false;
                    }
                }
            }
            None => return true,
        }
        true
    }
}
