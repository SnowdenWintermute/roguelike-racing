use super::abilities::{CombatantAbility, TargetingScheme, ValidTargets};
use crate::adventuring_party::AdventuringParty;

impl CombatantAbility {
    pub fn get_default_target_ids(
        &self,
        party: &AdventuringParty,
        character_id: u32,
    ) -> Option<Vec<u32>> {
        let targeting_scheme = &self.targeting_schemes[0];
        let valid_targets = &self.valid_targets;
        match valid_targets {
            ValidTargets::Opponent => match &party.current_room.monsters {
                Some(monsters) => {
                    let all_monster_ids = monsters
                        .iter()
                        .map(|monster| monster.entity_properties.id)
                        .collect::<Vec<u32>>();
                    match targeting_scheme {
                        TargetingScheme::Single => Some(vec![monsters[0].entity_properties.id]),
                        TargetingScheme::Area => Some(all_monster_ids),
                    }
                }
                None => None,
            },
            ValidTargets::User => Some(vec![character_id]),
            ValidTargets::Friendly => match targeting_scheme {
                TargetingScheme::Single => Some(vec![character_id]),
                TargetingScheme::Area => Some(party.character_positions.clone()),
            },
            ValidTargets::Any => match &party.current_room.monsters {
                Some(monsters) => {
                    let all_monster_ids = monsters
                        .iter()
                        .map(|monster| monster.entity_properties.id)
                        .collect::<Vec<u32>>();
                    match targeting_scheme {
                        super::abilities::TargetingScheme::Single => {
                            Some(vec![monsters[0].entity_properties.id])
                        }
                        super::abilities::TargetingScheme::Area => Some(all_monster_ids),
                    }
                }
                None => match targeting_scheme {
                    TargetingScheme::Single => Some(vec![character_id]),
                    TargetingScheme::Area => Some(party.character_positions.clone()),
                },
            },
        }
    }
}
