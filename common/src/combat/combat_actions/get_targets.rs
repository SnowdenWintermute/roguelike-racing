use super::CombatActionTarget;
use super::FriendOrFoe;
use super::TargetingScheme;
use crate::app_consts::error_messages;
use crate::errors::AppError;

impl CombatActionTarget {
    pub fn get_targets_if_scheme_valid(
        &self,
        ally_ids: Vec<u32>,
        opponent_ids_option: Option<Vec<u32>>,
        excluded_schemes: Vec<TargetingScheme>,
    ) -> Result<Vec<u32>, AppError> {
        let invalid_targeting_scheme_error = AppError {
            error_type: crate::errors::AppErrorTypes::InvalidInput,
            message: error_messages::INVALID_TARGETS_SELECTED.to_string(),
        };

        let targets = match self {
            CombatActionTarget::Single(id) => {
                if excluded_schemes.contains(&TargetingScheme::Single) {
                    return Err(invalid_targeting_scheme_error);
                }
                vec![*id]
            }
            CombatActionTarget::Group(category) => {
                if excluded_schemes.contains(&TargetingScheme::Area) {
                    return Err(invalid_targeting_scheme_error);
                }
                match category {
                    FriendOrFoe::Friendly => ally_ids,
                    FriendOrFoe::Hostile => opponent_ids_option.ok_or_else(|| AppError {
                        error_type: crate::errors::AppErrorTypes::Generic,
                        message: error_messages::NO_POSSIBLE_TARGETS_PROVIDED.to_string(),
                    })?,
                }
            }
            CombatActionTarget::All => {
                if excluded_schemes.contains(&TargetingScheme::All) {
                    return Err(invalid_targeting_scheme_error);
                }
                let mut to_return = vec![];
                to_return.append(&mut ally_ids.clone());
                if let Some(opponent_ids) = opponent_ids_option {
                    to_return.append(&mut opponent_ids.clone());
                }
                to_return
            }
        };

        Ok(targets)
    }
}
