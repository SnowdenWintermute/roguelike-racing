use super::get_combatant_ability_attributes::TargetCategories;
use super::get_combatant_ability_attributes::TargetingScheme;
use super::AbilityTarget;
use super::CombatantAbilityNames;
use super::FriendOrFoe;
use crate::app_consts::error_messages;
use crate::combatants::AbilityTargetPreferences;
use crate::errors::AppError;

impl CombatantAbilityNames {
    pub fn targets_by_saved_preference_or_default(
        &self,
        character_id: u32,
        target_preferences: &AbilityTargetPreferences,
        ally_ids: Vec<u32>,
        opponent_ids_option: Option<Vec<u32>>,
    ) -> Result<AbilityTarget, AppError> {
        let ability_attributes = self.get_attributes();
        if ability_attributes
            .targeting_schemes
            .contains(&target_preferences.targeting_scheme_preference)
        {
            targeting_scheme_targets_or_defaults(
                &target_preferences.targeting_scheme_preference,
                &ability_attributes.valid_target_categories,
                target_preferences,
                character_id,
                ally_ids,
                opponent_ids_option,
            )
        } else {
            let default_targeting_scheme = ability_attributes
                .targeting_schemes
                .first()
                .ok_or_else(|| AppError {
                    error_type: crate::errors::AppErrorTypes::ClientError,
                    message: error_messages::ABILITY_HAS_NO_TARGETING_SCHEME.to_string(),
                })?;
            targeting_scheme_targets_or_defaults(
                &default_targeting_scheme,
                &ability_attributes.valid_target_categories,
                target_preferences,
                character_id,
                ally_ids,
                opponent_ids_option,
            )
        }
    }
}

fn targeting_scheme_targets_or_defaults(
    targeting_scheme: &TargetingScheme,
    valid_target_categories: &TargetCategories,
    target_preferences: &AbilityTargetPreferences,
    character_id: u32,
    ally_ids: Vec<u32>,
    opponent_ids_option: Option<Vec<u32>>,
) -> Result<AbilityTarget, AppError> {
    match targeting_scheme {
        TargetingScheme::Single => match valid_target_categories {
            TargetCategories::Opponent => preferred_single_target_or_default(
                FriendOrFoe::Hostile,
                &target_preferences,
                ally_ids,
                opponent_ids_option,
            ),
            TargetCategories::User => Ok(AbilityTarget::Single(character_id)),
            TargetCategories::Friendly => preferred_single_target_or_default(
                FriendOrFoe::Friendly,
                &target_preferences,
                ally_ids,
                opponent_ids_option,
            ),
            TargetCategories::Any => match &target_preferences.category_of_last_target {
                Some(category) => match category {
                    FriendOrFoe::Friendly => preferred_single_target_or_default(
                        FriendOrFoe::Friendly,
                        &target_preferences,
                        ally_ids,
                        opponent_ids_option,
                    ),
                    FriendOrFoe::Hostile => preferred_single_target_or_default(
                        FriendOrFoe::Hostile,
                        &target_preferences,
                        ally_ids,
                        opponent_ids_option,
                    ),
                },
                None => {
                    if let Some(opponent_ids) = opponent_ids_option {
                        Ok(AbilityTarget::Single(opponent_ids[0]))
                    } else {
                        Ok(AbilityTarget::Single(ally_ids[0]))
                    }
                }
            },
        },
        TargetingScheme::Area => match valid_target_categories {
            TargetCategories::Opponent => Ok(AbilityTarget::Group(FriendOrFoe::Hostile)),
            TargetCategories::User => Ok(AbilityTarget::Single(character_id)),
            TargetCategories::Friendly => Ok(AbilityTarget::Group(FriendOrFoe::Friendly)),
            TargetCategories::Any => match &target_preferences.category_of_last_target {
                Some(category) => match category {
                    FriendOrFoe::Friendly => Ok(AbilityTarget::Group(FriendOrFoe::Friendly)),
                    FriendOrFoe::Hostile => Ok(AbilityTarget::Group(FriendOrFoe::Hostile)),
                },
                None => Ok(AbilityTarget::Group(FriendOrFoe::Hostile)),
            },
        },
        TargetingScheme::All => Ok(AbilityTarget::All),
    }
}

fn preferred_single_target_or_default(
    category: FriendOrFoe,
    target_preferences: &AbilityTargetPreferences,
    ally_ids: Vec<u32>,
    opponent_ids_option: Option<Vec<u32>>,
) -> Result<AbilityTarget, AppError> {
    match category {
        FriendOrFoe::Friendly => {
            let default_ally_id = ally_ids.first().ok_or_else(|| AppError {
                error_type: crate::errors::AppErrorTypes::ClientError,
                message: error_messages::ALLY_COMBATANTS_NOT_FOUND.to_string(),
            })?;
            if let Some(previously_targeted_id) = target_preferences.friendly_single {
                if ally_ids.contains(&previously_targeted_id) {
                    Ok(AbilityTarget::Single(previously_targeted_id))
                } else {
                    Ok(AbilityTarget::Single(*default_ally_id))
                }
            } else {
                Ok(AbilityTarget::Single(*default_ally_id))
            }
        }
        FriendOrFoe::Hostile => {
            let opponent_ids = opponent_ids_option.ok_or_else(|| AppError {
                error_type: crate::errors::AppErrorTypes::Generic,
                message: error_messages::ENEMY_COMBATANTS_NOT_FOUND.to_string(),
            })?;
            let default_opponent_id = opponent_ids.first().ok_or_else(|| AppError {
                error_type: crate::errors::AppErrorTypes::ClientError,
                message: error_messages::ENEMY_COMBATANTS_NOT_FOUND.to_string(),
            })?;
            if let Some(previously_targeted_id) = target_preferences.hostile_single {
                if opponent_ids.contains(&previously_targeted_id) {
                    Ok(AbilityTarget::Single(previously_targeted_id))
                } else {
                    Ok(AbilityTarget::Single(*default_opponent_id))
                }
            } else {
                Ok(AbilityTarget::Single(*default_opponent_id))
            }
        }
    }
}
