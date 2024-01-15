mod add_event_to_combat_log;
pub mod handle_animation_finished;
use crate::components::mesh_manager::ClientCombatantEvent;
use crate::store::alert_store::AlertStore;
use crate::store::game_store::GameStore;
use common::app_consts::error_messages;
use common::combat::CombatTurnResult;
use common::errors::AppError;
use common::utils::vec_shift;
use gloo::console::log;
use yewdux::Dispatch;

// store the results in a queue
// pass the first result to the entity and have them animate
//   -- approach
//   -- swing to contact
//   -- follow through swing
//     -- reduce hp
//     -- animate hit recovery
//     -- floating numbers
//   -- query queue for next result
//   -- swing to contact
//     -- reduce hp
//     -- animate hit recovery
//     -- floating numbers
//   -- follow through swing
//   -- return to spot
//   -- pass turn

pub fn handle_combat_turn_results(
    game_dispatch: Dispatch<GameStore>,
    alert_dispatch: Dispatch<AlertStore>,
    turn_results: Vec<CombatTurnResult>,
) -> Result<(), AppError> {
    log!(format!("got combat turn results: {:#?}", turn_results));
    for turn_result in turn_results {
        let CombatTurnResult { action_results, .. } = turn_result;
        for action_result in action_results.iter() {
            let cloned_dispatch = game_dispatch.clone();
            cloned_dispatch.reduce_mut(|store| {
                store
                    .action_results_manager
                    .turn_results_queue
                    .push(action_result.clone())
            });
        }
    }
    process_next_event_in_turn_result_queue(game_dispatch, alert_dispatch)?;
    Ok(())
}

pub fn process_next_event_in_turn_result_queue(
    game_dispatch: Dispatch<GameStore>,
    alert_dispatch: Dispatch<AlertStore>,
) -> Result<(), AppError> {
    let next_turn_action_result_option = game_dispatch
        .reduce_mut(|store| vec_shift(&mut store.action_results_manager.turn_results_queue));
    log!(format!(
        "starting processing of turn result: {:#?}",
        next_turn_action_result_option
    ));
    if let Some(action_result) = next_turn_action_result_option {
        game_dispatch.reduce_mut(|store| -> Result<(), AppError> {
            log!(format!(
                "process_next_event_in_turn_result_queue called: {:#?}",
                store.action_results_manager.turn_results_queue
            ));
            let cloned_dispatch = game_dispatch.clone();
            let cloned_alert_dispatch = alert_dispatch.clone();
            let action_user_event_manager = store
                .action_results_manager
                .combantant_event_managers
                .get_mut(&action_result.user_id)
                .ok_or_else(|| AppError {
                    error_type: common::errors::AppErrorTypes::ClientError,
                    message: error_messages::COMBANTANT_EVENT_MANAGER_NOT_FOUND.to_string(),
                })?;

            action_user_event_manager.current_event_processing =
                Some(ClientCombatantEvent::TookAction(action_result));

            action_user_event_manager.process_active_event(cloned_dispatch, cloned_alert_dispatch);

            Ok(())
        })?;
    }
    Ok(())
}
