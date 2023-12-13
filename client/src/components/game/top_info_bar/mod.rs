use crate::{
    components::game::turn_order_bar::TurnOrderBar,
    store::game_store::{get_current_party_option, GameStore},
};
use yew::prelude::*;
use yewdux::prelude::use_store;

#[function_component(TopInfoBar)]
pub fn game() -> Html {
    let (game_state, _) = use_store::<GameStore>();
    let party_option = get_current_party_option(&game_state);

    html!(
    <div class="border-b border-r border-l border-slate-400 bg-slate-700 w-full text-center" >
    if let Some(party) = party_option {
        if party.combatant_turn_trackers.is_some() {
            <TurnOrderBar />
        } else {
            <div class="m-2">
                {"Floor "}{party.current_floor}{", room "}{party.rooms_explored.on_current_floor}
                {": "}
                {format!("{}", party.current_room.room_type)}
            </div>
        }
    }
    </div>
    )
}
