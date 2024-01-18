use super::get_combatant_ability_attributes::ProhibitedTargetCombatantStates;
use crate::errors::AppError;
use crate::game::RoguelikeRacerGame;

pub fn filter_possible_target_ids_by_prohibited_combatant_states(
    game: &RoguelikeRacerGame,
    prohibited_combatant_states_option: Option<Vec<ProhibitedTargetCombatantStates>>,
    ally_ids: Vec<u32>,
    opponent_ids_option: Option<Vec<u32>>,
) -> Result<(Vec<u32>, Option<Vec<u32>>), AppError> {
    if let Some(prohibited_combatant_states) = prohibited_combatant_states_option {
        if prohibited_combatant_states.len() == 0 {
            return Ok((ally_ids, opponent_ids_option));
        }
        let valid_ally_ids = filter_target_id_group_by_prohibited_combatant_states(
            game,
            ally_ids,
            &prohibited_combatant_states,
        )?;
        let valid_opponent_ids_option = if let Some(opponent_ids) = opponent_ids_option {
            Some(filter_target_id_group_by_prohibited_combatant_states(
                game,
                opponent_ids,
                &prohibited_combatant_states,
            )?)
        } else {
            None
        };

        return Ok((valid_ally_ids, valid_opponent_ids_option));
    } else {
        return Ok((ally_ids, opponent_ids_option));
    }
}

fn filter_target_id_group_by_prohibited_combatant_states(
    game: &RoguelikeRacerGame,
    potential_ids: Vec<u32>,
    prohibited_combatant_states: &Vec<ProhibitedTargetCombatantStates>,
) -> Result<Vec<u32>, AppError> {
    let mut valid_ids = vec![];
    for target_id in potential_ids {
        let (_, combatant_properties) = game.get_combatant_by_id(&target_id)?;
        let mut target_is_prohibited = false;
        for combatant_state in prohibited_combatant_states {
            match combatant_state {
                ProhibitedTargetCombatantStates::Dead => {
                    if combatant_properties.hit_points == 0 {
                        target_is_prohibited = true;
                    }
                }
                ProhibitedTargetCombatantStates::Alive => {
                    if combatant_properties.hit_points != 0 {
                        target_is_prohibited = true;
                    }
                }
            }
        }
        if target_is_prohibited {
            continue;
        } else {
            valid_ids.push(target_id)
        }
    }
    Ok(valid_ids)
}
