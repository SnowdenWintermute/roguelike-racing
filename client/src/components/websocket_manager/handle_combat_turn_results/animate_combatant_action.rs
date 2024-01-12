use super::handle_animation_finished::handle_animation_finished;
use crate::store::game_store::GameStore;
use common::utils::vec_shift;
use yewdux::Dispatch;

// take the first result and give it to the appropriate entity's animation manager and play it
// when that animation finishes:
// - end the current battle's active combatant turn if appropriate
// - trigger the turn_results_animation_queue to pass the next
//   animation to the appropriate entity and play it

pub fn animate_next_combatant_action(game_dispatch: Dispatch<GameStore>) {
    game_dispatch.reduce_mut(|store| {
        // if let Some(action_result) =
        //     vec_shift(&mut store.animation_manager.turn_results_animation_queue)
        // {
        //     let combatant_animation_manager_option = store
        //         .animation_manager
        //         .combatant_animation_managers
        //         .get_mut(&action_result.user_id);

        //     if let Some(combatant_animation_manager) = combatant_animation_manager_option {
        //         // queue the animation in the combatant's animation manager
        //         combatant_animation_manager
        //             .action_result_animation_queue
        //             .push(action_result.clone());

        //         // if combatant not animating, set their next animation to start
        //         if combatant_animation_manager
        //             .current_action_result_animation
        //             .is_none()
        //         {
        //             if let Some(next_action_result) =
        //                 vec_shift(&mut combatant_animation_manager.action_result_animation_queue)
        //             {
        //                 combatant_animation_manager.current_action_result_animation =
        //                     Some(next_action_result);
        //                 let cloned_dispatch = game_dispatch.clone();
        //                 let _ = gloo::timers::callback::Timeout::new(1333, move || {
        //                     handle_animation_finished(cloned_dispatch, action_result.clone());
        //                 })
        //                 .forget();
        //             }
        //         }
        //     }
        // }
    });
}
