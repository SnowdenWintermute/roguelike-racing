use crate::store::game_store::GameStore;
use yew::prelude::*;
use yewdux::prelude::use_store;

#[function_component(GameDebug)]
pub fn game_debug() -> Html {
    let (game_state, _) = use_store::<GameStore>();

    let turn_results_queue = &game_state.action_results_manager.turn_results_queue;
    let turn_results_queue_display = turn_results_queue
        .iter()
        .map(|item| {
            html!(
                <div class="mr-2 last:mr-0" >
                {item.combatant_id}
                </div>
            )
        })
        .collect::<Html>();

    html!(
    <div class="z-10 absolute top-0 left- 0 bg-black text-windgreen p-2">
        <div>{"Debug"}</div>
        <div class="flex" >
           {turn_results_queue_display}
        </div>
    </div>
    )
}
