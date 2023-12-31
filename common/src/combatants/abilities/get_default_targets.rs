use super::{
    get_combatant_ability_attributes::{TargetCategories, TargetingScheme},
    AbilityTarget, CombatantAbilityNames, FriendOrFoe,
};
use crate::{app_consts::error_messages, errors::AppError};

impl CombatantAbilityNames {
    pub fn get_default_targets(
        &self,
        ability_user_id: u32,
        ally_ids: &Vec<u32>,
        opponent_ids_option: &Option<Vec<u32>>,
    ) -> Result<AbilityTarget, AppError> {
        let ability_attributes = self.get_attributes();
        let default_targeting_scheme =
            ability_attributes
                .targeting_schemes
                .first()
                .ok_or_else(|| AppError {
                    error_type: crate::errors::AppErrorTypes::ClientError,
                    message: error_messages::ABILITY_HAS_NO_TARGETING_SCHEME.to_string(),
                })?;

        match default_targeting_scheme {
            TargetingScheme::Single => match ability_attributes.valid_target_categories {
                TargetCategories::Opponent => {
                    let opponent_ids = opponent_ids_option.as_ref().ok_or_else(|| AppError {
                        error_type: crate::errors::AppErrorTypes::Generic,
                        message: error_messages::ENEMY_COMBATANTS_NOT_FOUND.to_string(),
                    })?;
                    let default_target = opponent_ids.first().ok_or_else(|| AppError {
                        error_type: crate::errors::AppErrorTypes::ServerError,
                        message: error_messages::ENEMY_COMBATANTS_NOT_FOUND.to_string(),
                    })?;
                    Ok(AbilityTarget::Single(*default_target))
                }
                TargetCategories::User => Ok(AbilityTarget::Single(ability_user_id)),
                TargetCategories::Friendly => {
                    let default_target = ally_ids.first().ok_or_else(|| AppError {
                        error_type: crate::errors::AppErrorTypes::ServerError,
                        message: error_messages::ALLY_COMBATANTS_NOT_FOUND.to_string(),
                    })?;
                    Ok(AbilityTarget::Single(*default_target))
                }
                TargetCategories::Any => {
                    if let Some(opponent_ids) = opponent_ids_option {
                        let default_target = opponent_ids.first().ok_or_else(|| AppError {
                            error_type: crate::errors::AppErrorTypes::ServerError,
                            message: error_messages::ENEMY_COMBATANTS_NOT_FOUND.to_string(),
                        })?;
                        Ok(AbilityTarget::Single(*default_target))
                    } else {
                        let default_target = ally_ids.first().ok_or_else(|| AppError {
                            error_type: crate::errors::AppErrorTypes::ServerError,
                            message: error_messages::ALLY_COMBATANTS_NOT_FOUND.to_string(),
                        })?;
                        Ok(AbilityTarget::Single(*default_target))
                    }
                }
            },
            TargetingScheme::Area => match ability_attributes.valid_target_categories {
                TargetCategories::Opponent => Ok(AbilityTarget::Group(FriendOrFoe::Hostile)),
                TargetCategories::User => Ok(AbilityTarget::Single(ability_user_id)),
                TargetCategories::Friendly => Ok(AbilityTarget::Group(FriendOrFoe::Friendly)),
                TargetCategories::Any => Ok(AbilityTarget::Group(FriendOrFoe::Friendly)),
            },
            TargetingScheme::All => Ok(AbilityTarget::All),
        }
    }
}
