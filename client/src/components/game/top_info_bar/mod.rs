use crate::store::game_store::{get_current_party_option, GameStore};
use yew::prelude::*;
use yewdux::prelude::use_store;

#[function_component(TopInfoBar)]
pub fn game() -> Html {
    let (game_state, _) = use_store::<GameStore>();
    let party_option = get_current_party_option(&game_state);

    html!(
    <div class="p-2 border-b border-r border-l border-slate-400 bg-slate-700 w-full text-center" >
    if let Some(party) = party_option {
        {"Floor "}{party.current_floor}{", room "}{party.rooms_explored.on_current_floor}
        {": "}
        {format!("{}", party.current_room.room_type)}
        }
    </div>
    )
}
