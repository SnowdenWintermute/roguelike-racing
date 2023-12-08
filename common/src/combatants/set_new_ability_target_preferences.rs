use super::{
    abilities::{
        get_combatant_ability_attributes::TargetingScheme,
        targets_by_saved_preference_or_default::is_id_of_existing_opponent, AbilityTarget,
        CombatantAbilityNames, FriendOrFoe,
    },
    AbilityTargetPreferences,
};
use crate::adventuring_party::AdventuringParty;

impl AbilityTargetPreferences {
    pub fn get_updated_preferences(
        &self,
        selected_ability_name: &CombatantAbilityNames,
        new_targets: &AbilityTarget,
        party: &AdventuringParty,
    ) -> AbilityTargetPreferences {
        let mut new_preferences = self.clone();
        let ability_attributes = selected_ability_name.get_attributes();
        match new_targets {
            AbilityTarget::Single(id) => {
                if is_id_of_existing_opponent(party, &id) {
                    new_preferences.hostile_single = Some(*id);
                    new_preferences.category_of_last_single = Some(FriendOrFoe::Hostile);
                } else if party.character_positions.contains(&id) {
                    new_preferences.friendly_single = Some(*id);
                    new_preferences.category_of_last_single = Some(FriendOrFoe::Friendly);
                }
            }
            AbilityTarget::Group(category) => {
                if ability_attributes.targeting_schemes.len() > 1 {
                    new_preferences.category_of_last_area = Some(category.clone());
                    new_preferences.targeting_scheme_preference = TargetingScheme::Area;
                } else {
                    // they had no choice, don't update prefs
                }
            }
            AbilityTarget::All => {
                if ability_attributes.targeting_schemes.len() > 1 {
                    new_preferences.targeting_scheme_preference = TargetingScheme::All;
                } else {
                    // they had no choice, don't update prefs
                }
            }
        }
        new_preferences
    }
}
