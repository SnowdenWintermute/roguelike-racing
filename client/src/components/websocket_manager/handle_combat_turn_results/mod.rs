mod animate_combatant_action;
mod handle_animation_finished;
use crate::components::mesh_manager::ClientCombatantEvent;
use crate::store::game_store::get_current_battle_option;
use crate::store::game_store::GameStore;
use common::app_consts::error_messages;
use common::combat::CombatTurnResult;
use common::combatants::abilities::AbilityTarget;
use common::combatants::abilities::FriendOrFoe;
use common::errors::AppError;
use common::utils::vec_shift;
use yewdux::Dispatch;

pub fn handle_combat_turn_results(
    game_dispatch: Dispatch<GameStore>,
    turn_results: Vec<CombatTurnResult>,
) -> Result<(), AppError> {
    for turn_result in turn_results {
        let CombatTurnResult { action_results, .. } = turn_result;
        for action_result in action_results.iter() {
            // push the action results into the animation manager
            let cloned_dispatch = game_dispatch.clone();
            game_dispatch.reduce_mut(|store| {
                store
                    .action_results_manager
                    .turn_results_queue
                    .push(action_result.clone())
            });
            // process action results queue
            game_dispatch.reduce_mut(|store| -> Result<(), AppError> {
                if let Some(action_result) =
                    vec_shift(&mut store.action_results_manager.turn_results_queue)
                {
                    let action_user_event_manager = store
                        .action_results_manager
                        .combantant_event_managers
                        .get_mut(&action_result.user_id)
                        .ok_or_else(|| AppError {
                            error_type: common::errors::AppErrorTypes::ClientError,
                            message: error_messages::COMBANTANT_EVENT_MANAGER_NOT_FOUND.to_string(),
                        })?;

                    action_user_event_manager
                        .event_queue
                        .push(ClientCombatantEvent::TookAction(action_result.clone()));

                    if action_user_event_manager.current_event_processing.is_none() {
                        // process the first event in their queue
                        if let Some(event) = vec_shift(&mut action_user_event_manager.event_queue) {
                            // start animation
                            action_user_event_manager.current_event_processing =
                                Some(event.clone());
                            let cloned_event = event.clone();
                            gloo::timers::callback::Timeout::new(1500, move || {
                                handle_animation_finished::handle_animation_finished(
                                    cloned_event,
                                    cloned_dispatch,
                                );
                            })
                            .forget();
                        }
                    }
                }
                Ok(())
            })?
        }
    }

    Ok(())
}
