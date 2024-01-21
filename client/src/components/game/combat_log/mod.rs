use crate::store::game_store::GameStore;
use yew::prelude::*;
use yewdux::use_store;

#[function_component(CombatLog)]
pub fn combat_log() -> Html {
    let (game_state, _) = use_store::<GameStore>();

    html!(
        <div class="h-full flex flex-col">
            <h3 class="flex-grow-0 flex-shrink" >{"Combat log"}</h3>
            <div class="list-none overflow-y-auto
           flex flex-col-reverse flex-1" >
               <ul class="" >
               {game_state.combat_log.iter().map(|log_entry| html!(<li>{log_entry}</li>)).collect::<Html>()}
               </ul>
            </div>
        </div>
    )
}
