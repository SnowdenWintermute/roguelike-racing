use super::CombatActionProperties;
use super::CombatActionTarget;
use super::FriendOrFoe;
use super::TargetCategories;
use super::TargetingScheme;
use crate::app_consts::error_messages;
use crate::combatants::CombatActionTargetPreferences;
use crate::errors::AppError;

// PREFS
// pub friendly_single: Option<u32>,
// pub hostile_single: Option<u32>,
// pub category_of_last_target: Option<FriendOrFoe>,
// pub category_of_last_area: Option<FriendOrFoe>,
// pub targeting_scheme_preference: TargetingScheme,

// CATEGORIES
// Friendly,
// Hostile,

// SCHEMES
// Single,
// Area,
// All,

// TARGETS
// Single(u32),
// Group(FriendOrFoe),
// All,

// ON SELECT ABILITY
// IF SchemePreference available on SelectedAbility
//  - IF Scheme == Single
//    - if single target preference in that category is valid, select them
//    - ELSE check for other valid Single ids in the preferred Category and
//  select the first
//  - ELSE if valid targets exist for Scheme in preferred category, select them
//  - ELSE IF ability allows another category
//  FOR EACH other Category
//  - IF Scheme == Single
//    - if single target preference in that category is valid, select them
//    - ELSE check for other valid Single ids in the preferred Category and
//  select the first
//  - ELSE if valid targets exist for Scheme in this category, select them

//  ELSE IF another scheme is available on the SelectedAbility
//  - IF Scheme == Single
//    - if single target preference in that category is valid, select them
//    - ELSE check for other valid Single ids in the preferred Category and
//  select the first
//   - ELSE IF valid targets exist for that scheme in the preferred category, select them
//   - ELSE IF another category exists
//      - IF Scheme == Single
//        - IF single target preference in that category is valid, select them
//        - ELSE check for other valid Single ids in the preferred Category and
//   - ELSE if valid targets exist in that category, select them

impl CombatActionProperties {
    pub fn targets_by_saved_preference_or_default(
        &self,
        character_id: u32,
        target_preferences: &CombatActionTargetPreferences,
        ally_ids: &Vec<u32>,
        opponent_ids_option: &Option<Vec<u32>>,
    ) -> Result<CombatActionTarget, AppError> {
        if self
            .targeting_schemes
            .contains(&target_preferences.targeting_scheme_preference)
        {
            targeting_scheme_targets_or_defaults(
                &target_preferences.targeting_scheme_preference,
                &self.valid_target_categories,
                target_preferences,
                character_id,
                &ally_ids,
                &opponent_ids_option,
            )
        } else {
            let default_targeting_scheme =
                self.targeting_schemes.first().ok_or_else(|| AppError {
                    error_type: crate::errors::AppErrorTypes::ClientError,
                    message: error_messages::ABILITY_HAS_NO_TARGETING_SCHEME.to_string(),
                })?;
            targeting_scheme_targets_or_defaults(
                &default_targeting_scheme,
                &self.valid_target_categories,
                target_preferences,
                character_id,
                &ally_ids,
                &opponent_ids_option,
            )
        }
    }
}

fn targeting_scheme_targets_or_defaults(
    targeting_scheme: &TargetingScheme,
    valid_target_categories: &TargetCategories,
    target_preferences: &CombatActionTargetPreferences,
    character_id: u32,
    ally_ids: &Vec<u32>,
    opponent_ids_option: &Option<Vec<u32>>,
) -> Result<CombatActionTarget, AppError> {
    match targeting_scheme {
        TargetingScheme::Single => match valid_target_categories {
            TargetCategories::Opponent => preferred_single_target_or_default(
                FriendOrFoe::Hostile,
                &target_preferences,
                character_id,
                &ally_ids,
                &opponent_ids_option,
            ),
            TargetCategories::User => Ok(CombatActionTarget::Single(character_id)),
            TargetCategories::Friendly => preferred_single_target_or_default(
                FriendOrFoe::Friendly,
                &target_preferences,
                character_id,
                &ally_ids,
                &opponent_ids_option,
            ),
            TargetCategories::Any => match &target_preferences.category_of_last_target {
                Some(category) => match category {
                    FriendOrFoe::Friendly => preferred_single_target_or_default(
                        FriendOrFoe::Friendly,
                        &target_preferences,
                        character_id,
                        &ally_ids,
                        &opponent_ids_option,
                    ),
                    FriendOrFoe::Hostile => preferred_single_target_or_default(
                        FriendOrFoe::Hostile,
                        &target_preferences,
                        character_id,
                        &ally_ids,
                        &opponent_ids_option,
                    ),
                },
                None => {
                    if let Some(opponent_ids) = opponent_ids_option {
                        Ok(CombatActionTarget::Single(opponent_ids[0]))
                    } else {
                        Ok(CombatActionTarget::Single(ally_ids[0]))
                    }
                }
            },
        },
        TargetingScheme::Area => match valid_target_categories {
            TargetCategories::Opponent => Ok(CombatActionTarget::Group(FriendOrFoe::Hostile)),
            TargetCategories::User => Ok(CombatActionTarget::Single(character_id)),
            TargetCategories::Friendly => Ok(CombatActionTarget::Group(FriendOrFoe::Friendly)),
            TargetCategories::Any => match &target_preferences.category_of_last_target {
                Some(category) => match category {
                    FriendOrFoe::Friendly => Ok(CombatActionTarget::Group(FriendOrFoe::Friendly)),
                    FriendOrFoe::Hostile => Ok(CombatActionTarget::Group(FriendOrFoe::Hostile)),
                },
                None => Ok(CombatActionTarget::Group(FriendOrFoe::Hostile)),
            },
        },
        TargetingScheme::All => Ok(CombatActionTarget::All),
    }
}

fn preferred_single_target_or_default(
    category: FriendOrFoe,
    target_preferences: &CombatActionTargetPreferences,
    character_id: u32,
    ally_ids: &Vec<u32>,
    opponent_ids_option: &Option<Vec<u32>>,
) -> Result<CombatActionTarget, AppError> {
    match category {
        FriendOrFoe::Friendly => {
            let default_ally_id = character_id;
            if let Some(previously_targeted_id) = target_preferences.friendly_single {
                if ally_ids.contains(&previously_targeted_id) {
                    Ok(CombatActionTarget::Single(previously_targeted_id))
                } else {
                    Ok(CombatActionTarget::Single(default_ally_id))
                }
            } else {
                Ok(CombatActionTarget::Single(default_ally_id))
            }
        }
        FriendOrFoe::Hostile => {
            let opponent_ids = opponent_ids_option.as_ref().ok_or_else(|| AppError {
                error_type: crate::errors::AppErrorTypes::Generic,
                message: error_messages::ENEMY_COMBATANTS_NOT_FOUND.to_string(),
            })?;
            let default_opponent_id = opponent_ids.first().ok_or_else(|| AppError {
                error_type: crate::errors::AppErrorTypes::ClientError,
                message: error_messages::NO_VALID_TARGETS_FOUND.to_string(),
            })?;
            if let Some(previously_targeted_id) = target_preferences.hostile_single {
                if opponent_ids.contains(&previously_targeted_id) {
                    Ok(CombatActionTarget::Single(previously_targeted_id))
                } else {
                    Ok(CombatActionTarget::Single(*default_opponent_id))
                }
            } else {
                Ok(CombatActionTarget::Single(*default_opponent_id))
            }
        }
    }
}
