use super::abilities::get_combatant_ability_attributes::TargetingScheme;
use super::abilities::AbilityTarget;
use super::abilities::CombatantAbilityNames;
use super::abilities::FriendOrFoe;
use super::AbilityTargetPreferences;

impl AbilityTargetPreferences {
    pub fn get_updated_preferences(
        &self,
        selected_ability_name: &CombatantAbilityNames,
        new_targets: &AbilityTarget,
        ally_ids: Vec<u32>,
        opponent_ids_option: Option<Vec<u32>>,
    ) -> AbilityTargetPreferences {
        let mut new_preferences = self.clone();
        let ability_attributes = selected_ability_name.get_attributes();
        match new_targets {
            AbilityTarget::Single(id) => {
                let is_opponent_id = {
                    if let Some(opponent_ids) = opponent_ids_option {
                        opponent_ids.contains(id)
                    } else {
                        false
                    }
                };
                if is_opponent_id {
                    new_preferences.hostile_single = Some(*id);
                    new_preferences.category_of_last_target = Some(FriendOrFoe::Hostile);
                } else if ally_ids.contains(&id) {
                    new_preferences.friendly_single = Some(*id);
                    new_preferences.category_of_last_target = Some(FriendOrFoe::Friendly);
                }
            }
            AbilityTarget::Group(category) => {
                if ability_attributes.targeting_schemes.len() > 1 {
                    new_preferences.category_of_last_target = Some(category.clone());
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
