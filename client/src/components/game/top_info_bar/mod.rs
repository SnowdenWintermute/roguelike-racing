use crate::components::game::turn_order_bar::TurnOrderBar;
use crate::store::game_store::get_current_battle_option;
use crate::store::game_store::get_current_party_option;
use crate::store::game_store::GameStore;
use yew::prelude::*;
use yewdux::prelude::use_store;

#[function_component(TopInfoBar)]
pub fn game() -> Html {
    let (game_state, _) = use_store::<GameStore>();
    let battle_option = get_current_battle_option(&game_state);
    let party_option = get_current_party_option(&game_state);

    html!(
    <div class="border-b border-r border-l border-slate-400 bg-slate-700 w-full text-center" >
        if let Some(battle) = battle_option {
                <TurnOrderBar />
            } else if let Some(party) =party_option{
                <div class="m-2">
                    {"Floor "}{party.current_floor}{", room "}{party.rooms_explored.on_current_floor}
                    {": "}
                    {format!("{}", party.current_room.room_type)}
                </div>
            } else {
                <div>{"Error - no party found"}</div>
            }
    </div>
    )
}
