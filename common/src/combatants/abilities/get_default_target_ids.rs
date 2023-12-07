use super::{
    get_combatant_ability_attributes::TargetCategories, CombatantAbility, TargetingScheme,
};
use crate::adventuring_party::AdventuringParty;

impl CombatantAbility {
    pub fn get_default_target_ids(
        &self,
        party: &AdventuringParty,
        character_id: u32,
    ) -> Option<Vec<u32>> {
        let ability_attributes = &self.ability_name.get_attributes();
        let targeting_scheme = &ability_attributes.targeting_schemes[0];
        let valid_targets = &ability_attributes.valid_target_categories;
        match valid_targets {
            TargetCategories::Opponent => match &party.current_room.monsters {
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
            TargetCategories::User => Some(vec![character_id]),
            TargetCategories::Friendly => match targeting_scheme {
                TargetingScheme::Single => Some(vec![character_id]),
                TargetingScheme::Area => Some(party.character_positions.clone()),
            },
            TargetCategories::Any => match &party.current_room.monsters {
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
                None => match targeting_scheme {
                    TargetingScheme::Single => Some(vec![character_id]),
                    TargetingScheme::Area => Some(party.character_positions.clone()),
                },
            },
        }
    }
}
