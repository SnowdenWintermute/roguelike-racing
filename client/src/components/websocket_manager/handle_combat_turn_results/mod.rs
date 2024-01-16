mod add_event_to_combat_log;
use crate::store::game_store::GameStore;
use common::app_consts::error_messages;
use common::combat::CombatTurnResult;
use common::errors::AppError;
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
//
//   Entities have:
//   current action result processing (if any)
//   when an action result is passed, start animating
//    - push to a queue of animations (move, swing to hit [damage here], follow through, recover, return)
//    - animations have an on_finish which can trigger animations on other entities, interrupting
//    their current hit recovery animation if any (getting hit before hit recovery animation finishes). Trigger
//    the on_finish for that animation (floating numbers, combat log entry)
//    - if take damage while in an action animation, just reduce the hp and show the floating
//    numbers
//    - if die while in an action animation, show floating numbers and, play the death animation in place
//    - entities can not select (or execute) a new action until their animaton queues are finished
//

pub fn handle_combat_turn_results(
    game_dispatch: Dispatch<GameStore>,
    turn_results: Vec<CombatTurnResult>,
) -> Result<(), AppError> {
    log!(format!("got combat turn results: {:#?}", turn_results));
    game_dispatch.reduce_mut(|store| {
        for turn_result in turn_results {
            store
                .action_results_manager
                .turn_results_queue
                .push_back(turn_result)
        }
    });
    send_next_turn_result_to_combatant_event_manager(game_dispatch.clone())
}

pub fn send_next_turn_result_to_combatant_event_manager(
    game_dispatch: Dispatch<GameStore>,
) -> Result<(), AppError> {
    log!("attempting to send_next_turn_result_to_combatant_event_manager");
    game_dispatch.reduce_mut(|store| -> Result<(), AppError> {
        let next_turn_to_process_option =
            store.action_results_manager.turn_results_queue.pop_front();
        if let Some(next_turn_to_process) = next_turn_to_process_option {
            log!("next turn result found, sending to combatant...");
            for action_result in next_turn_to_process.action_results {
                store
                    .action_results_manager
                    .combantant_event_managers
                    .get_mut(&next_turn_to_process.combatant_id)
                    .ok_or_else(|| AppError {
                        error_type: common::errors::AppErrorTypes::ClientError,
                        message: error_messages::COMBANTANT_EVENT_MANAGER_NOT_FOUND.to_string(),
                    })?
                    .action_result_queue
                    .push_back(action_result);
            }
        }
        Ok(())
    })
}
