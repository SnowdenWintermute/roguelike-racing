use super::get_combatant_ability_attributes::TargetCategories;
use super::AbilityTarget;
use super::CombatantAbilityNames;
use super::FriendOrFoe;
use crate::app_consts::error_messages;
use crate::errors::AppError;
use crate::game::RoguelikeRacerGame;
use crate::primatives::NextOrPrevious;

impl CombatantAbilityNames {
    pub fn get_next_or_previous_targets(
        &self,
        current_targets: &AbilityTarget,
        direction: &NextOrPrevious,
        ability_user_id: &u32,
        ally_ids: &Vec<u32>,
        opponent_ids_option: &Option<Vec<u32>>,
        game: &RoguelikeRacerGame
    ) -> Result<AbilityTarget, AppError> {
        let ability_attributes = self.get_attributes();

        match current_targets {
            AbilityTarget::Single(id) => match ability_attributes.valid_target_categories {
                TargetCategories::Opponent => {
                    let possible_target_ids =
                        opponent_ids_option.clone().ok_or_else(|| AppError {
                            error_type: crate::errors::AppErrorTypes::Generic,
                            message: error_messages::ENEMY_COMBATANTS_NOT_FOUND.to_string(),
                        })?;
                    let new_target_id = get_next_or_prev_id_from_ordered_id_list(
                        &possible_target_ids,
                        *id,
                        &direction,
                    )?;
                    Ok(AbilityTarget::Single(new_target_id))
                }
                TargetCategories::User => Ok(AbilityTarget::Single(*ability_user_id)),
                TargetCategories::Friendly => {
                    let new_target_id =
                        get_next_or_prev_id_from_ordered_id_list(&ally_ids, *id, &direction)?;
                    Ok(AbilityTarget::Single(new_target_id))
                }
                TargetCategories::Any => {
                    let mut possible_target_ids = vec![];
                    if let Some(opponent_ids) = opponent_ids_option {
                        possible_target_ids.extend(opponent_ids);
                    }
                    possible_target_ids.extend(ally_ids.clone());
                    let new_target_id = get_next_or_prev_id_from_ordered_id_list(
                        &possible_target_ids,
                        *id,
                        &direction,
                    )?;
                    Ok(AbilityTarget::Single(new_target_id))
                }
            },
            AbilityTarget::Group(category) => match ability_attributes.valid_target_categories {
                TargetCategories::Opponent => Ok(AbilityTarget::Group(FriendOrFoe::Hostile)),
                TargetCategories::User => Ok(AbilityTarget::Single(*ability_user_id)),
                TargetCategories::Friendly => Ok(AbilityTarget::Group(FriendOrFoe::Friendly)),
                TargetCategories::Any => match category {
                    FriendOrFoe::Friendly => Ok(AbilityTarget::Group(FriendOrFoe::Hostile)),
                    FriendOrFoe::Hostile => Ok(AbilityTarget::Group(FriendOrFoe::Friendly)),
                },
            },
            AbilityTarget::All => Ok(AbilityTarget::All),
        }
    }
}

/// if the provided list of possible ids doesn't include the current target id, it will return the
/// id of the 0th indexed element
fn get_next_or_prev_id_from_ordered_id_list(
    possible_target_ids: &Vec<u32>,
    current_target_id: u32,
    direction: &NextOrPrevious,
) -> Result<u32, AppError> {
    let current_position_index = {
        let mut to_return = 0;
        for (index, id) in possible_target_ids.iter().enumerate() {
            if id == &current_target_id {
                to_return = index;
                break;
            }
        }
        to_return
    };

    let new_index = match direction {
        NextOrPrevious::Next => {
            if current_position_index < possible_target_ids.len() - 1 {
                current_position_index + 1
            } else {
                0
            }
        }
        NextOrPrevious::Previous => {
            if current_position_index > 0 {
                current_position_index - 1
            } else {
                possible_target_ids.len() - 1
            }
        }
    };

    let new_id = possible_target_ids.get(new_index).ok_or_else(|| AppError {
        error_type: crate::errors::AppErrorTypes::Generic,
        message: error_messages::NO_POSSIBLE_TARGETS_PROVIDED.to_string(),
    })?;
    Ok(*new_id)
}
