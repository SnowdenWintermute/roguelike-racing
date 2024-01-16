mod queue_attack_animations;
mod queue_melee_ability_animations;
mod queue_return_to_home_position_animations;
use self::queue_attack_animations::queue_attack_animations;
use crate::store::game_store::GameStore;
use common::combat::ActionResult;
use common::combat::CombatAction;
use common::combatants::abilities::CombatantAbilityNames;
use common::errors::AppError;
use yewdux::Dispatch;

pub fn process_next_action_result_in_combatant_event_queue(
    game_dispatch: Dispatch<GameStore>,
    current_action_processing: &Option<ActionResult>,
    combatant_id: u32,
) -> Result<(), AppError> {
    if let Some(new_action_result) = &current_action_processing {
        match &new_action_result.action {
            CombatAction::AbilityUsed(ability_name) => {
                match ability_name.get_attributes().is_melee {
                    true => queue_melee_ability_animations::queue_melee_ability_animations(
                        game_dispatch.clone(),
                        combatant_id,
                        new_action_result,
                    )?,
                    false => (),
                };
                match ability_name {
                    CombatantAbilityNames::Attack => {
                        queue_attack_animations(game_dispatch, combatant_id, new_action_result)
                    }
                    _ => Ok(()),
                }
            }
            CombatAction::ItemUsed(_) => {
                todo!()
            }
        }
    } else {
        queue_return_to_home_position_animations::queue_return_to_home_position_animations(
            game_dispatch,
            combatant_id,
        )
    }
}
