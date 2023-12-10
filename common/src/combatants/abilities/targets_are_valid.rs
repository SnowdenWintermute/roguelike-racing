use super::{
    get_combatant_ability_attributes::{TargetCategories, TargetingScheme},
    targets_by_saved_preference_or_default::is_id_of_existing_opponent,
    AbilityTarget, CombatantAbilityNames, FriendOrFoe,
};
use crate::adventuring_party::AdventuringParty;

impl CombatantAbilityNames {
    pub fn targets_are_valid(
        &self,
        character_id: u32,
        targets: &AbilityTarget,
        party: &AdventuringParty,
    ) -> bool {
        let ability_attributes = self.get_attributes();
        match targets {
            AbilityTarget::Single(id) => match ability_attributes.valid_target_categories {
                TargetCategories::Opponent => is_id_of_existing_opponent(party, id),
                TargetCategories::User => id == &character_id,
                TargetCategories::Friendly => party.character_positions.contains(&id),
                TargetCategories::Any => {
                    party.character_positions.contains(&id) || is_id_of_existing_opponent(party, id)
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
