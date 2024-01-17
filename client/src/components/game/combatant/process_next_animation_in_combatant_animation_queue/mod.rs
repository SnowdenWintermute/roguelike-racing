mod approach_combatant_animation_finished_handler;
mod follow_through_swing_animation_finished_handler;
mod handle_animation_finished;
mod return_to_ready_position_animation_finished_handler;
mod swing_to_hit_animation_finished_handler;
use crate::components::mesh_manager::CombatantAnimation;
use crate::store::game_store::GameStore;
use gloo::timers::callback::Timeout;
use std::collections::VecDeque;
use yew::UseStateHandle;
use yewdux::Dispatch;

pub fn process_next_animation_in_combatant_animation_queue(
    game_dispatch: Dispatch<GameStore>,
    animation_queue: &VecDeque<CombatantAnimation>,
    timer_state: UseStateHandle<Option<Timeout>>,
    combatant_id: u32,
) {
    if let Some(animation) = animation_queue.front() {
        let cloned_animation = animation.clone();
        timer_state.set(Some(gloo::timers::callback::Timeout::new(
            1500,
            move || {
                let result = handle_animation_finished::handle_animation_finished(
                    game_dispatch,
                    cloned_animation,
                    combatant_id,
                );
            },
        )));
    }
}
