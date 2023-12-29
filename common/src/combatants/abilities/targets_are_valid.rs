use super::{
    get_combatant_ability_attributes::{TargetCategories, TargetingScheme},
    AbilityTarget, CombatantAbilityNames, FriendOrFoe,
};

impl CombatantAbilityNames {
    pub fn targets_are_valid(
        &self,
        ability_user_id: u32,
        targets: &AbilityTarget,
        ally_ids: &Vec<u32>,
        opponent_ids_option: &Option<Vec<u32>>,
    ) -> bool {
        let ability_attributes = self.get_attributes();
        match targets {
            AbilityTarget::Single(id) => match ability_attributes.valid_target_categories {
                TargetCategories::Opponent => {
                    if let Some(opponent_ids) = opponent_ids_option {
                        opponent_ids.contains(id)
                    } else {
                        false
                    }
                }
                TargetCategories::User => id == &ability_user_id,
                TargetCategories::Friendly => ally_ids.contains(&id),
                TargetCategories::Any => {
                    ally_ids.contains(&id) || {
                        if let Some(opponent_ids) = opponent_ids_option {
                            opponent_ids.contains(id)
                        } else {
                            false
                        }
                    }
                }
            },
            AbilityTarget::Group(category) => {
                if ability_attributes
                    .targeting_schemes
                    .contains(&TargetingScheme::Area)
                {
                    match ability_attributes.valid_target_categories {
                        TargetCategories::Opponent => category == &FriendOrFoe::Hostile,
                        TargetCategories::User => false,
                        TargetCategories::Friendly => category == &FriendOrFoe::Friendly,
                        TargetCategories::Any => {
                            category == &FriendOrFoe::Hostile || category == &FriendOrFoe::Friendly
                        }
                    }
                } else {
                    false
                }
            }
            AbilityTarget::All => ability_attributes
                .targeting_schemes
                .contains(&TargetingScheme::All),
        }
    }
}
