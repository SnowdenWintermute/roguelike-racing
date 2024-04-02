use crate::yew_app::components::game::turn_order_bar::TurnOrderBar;
use crate::yew_app::store::game_store::get_current_battle_option;
use crate::yew_app::store::game_store::get_current_party_option;
use crate::yew_app::store::game_store::GameStore;
use yew::prelude::*;
use yewdux::prelude::use_store;

#[function_component(TopInfoBar)]
pub fn game() -> Html {
    let (game_state, _) = use_store::<GameStore>();
    let battle_option = get_current_battle_option(&game_state);
    let party_option = get_current_party_option(&game_state);

    html!(
    <div class="h-10 w-full mb-4 border border-slate-400 bg-slate-700 flex justify-between" >
    if let Some(party) = party_option {
        <div class="p-2" >
            {"Floor "}{party.current_floor}{", room "}{party.rooms_explored.on_current_floor}
            {": "}
            {format!("{}", party.current_room.room_type)}
        </div>
        if battle_option.is_some() { <TurnOrderBar /> }
            else { <RoomExplorationTracker /> }
        } else {
            <div>{"Error - no party found"}</div>
        }
    </div>
    )
}

#[function_component(RoomExplorationTracker)]
fn room_exploration_tracker() -> Html {
    let (game_state, _) = use_store::<GameStore>();
    let party_option = get_current_party_option(&game_state);
    if let Some(party) = party_option {
        html!(
        <ul class="list-none flex border-l border-slate-400" >
            {party.client_current_floor_rooms_list
                .iter()
                .enumerate()
                .map(|(i, room_type_option )| {
                    let current_room_class = if party.rooms_explored.on_current_floor == ( i + 1 ) as u16 {
                        "border border-yellow-400"
                    } else {
                        "border-slate-400"
                    };

                    html!(
                        <li class={ format!("p-2 border-l {}", current_room_class) }>
                        {match room_type_option {
                            Some(room_type) => {format!("{room_type}")},
                            None => { "?".to_string() },
                        }}
                        </li>
                        )
                })
                .collect::<Html>()}
        </ul>
        )
    } else {
        html!()
    }
}
