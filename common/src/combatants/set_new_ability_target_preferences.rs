use super::CombatActionTargetPreferences;
use crate::combat::combat_actions::CombatActionProperties;
use crate::combat::combat_actions::CombatActionTarget;
use crate::combat::combat_actions::FriendOrFoe;
use crate::combat::combat_actions::TargetingScheme;

impl CombatActionTargetPreferences {
    pub fn get_updated_preferences(
        &self,
        combat_action_properties: &CombatActionProperties,
        new_targets: &CombatActionTarget,
        ally_ids: Vec<u32>,
        opponent_ids_option: Option<Vec<u32>>,
    ) -> CombatActionTargetPreferences {
        let mut new_preferences = self.clone();
        match new_targets {
            CombatActionTarget::Single(id) => {
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
            CombatActionTarget::Group(category) => {
                if combat_action_properties.targeting_schemes.len() > 1 {
                    new_preferences.category_of_last_target = Some(category.clone());
                    new_preferences.targeting_scheme_preference = TargetingScheme::Area;
                } else {
                    // they had no choice, don't update prefs
                }
            }
            CombatActionTarget::All => {
                if combat_action_properties.targeting_schemes.len() > 1 {
                    new_preferences.targeting_scheme_preference = TargetingScheme::All;
                } else {
                    // they had no choice, don't update prefs
                }
            }
        }
        new_preferences
    }
}
