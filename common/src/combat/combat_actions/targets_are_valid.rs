use super::CombatActionProperties;
use super::CombatActionTarget;
use super::FriendOrFoe;
use super::TargetCategories;
use super::TargetingScheme;

impl CombatActionProperties {
    pub fn targets_are_valid(
        &self,
        ability_user_id: u32,
        targets: &CombatActionTarget,
        ally_ids: &Vec<u32>,
        opponent_ids_option: &Option<Vec<u32>>,
    ) -> bool {
        println!("targets for combat action: {:#?}", targets);
        match targets {
            CombatActionTarget::Single(id) => match self.valid_target_categories {
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
            CombatActionTarget::Group(category) => {
                if self.targeting_schemes.contains(&TargetingScheme::Area) {
                    match self.valid_target_categories {
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
            CombatActionTarget::All => self.targeting_schemes.contains(&TargetingScheme::All),
        }
    }
}
