mod turn_order_bar;
mod turn_order_tracker_icon;
use crate::yew_app::components::game::tailwind_class_loader::BUTTON_HEIGHT_SMALL;
use crate::yew_app::components::game::tailwind_class_loader::SPACING_REM;
use crate::yew_app::components::game::top_info_bar::turn_order_bar::TurnOrderBar;
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

    if let Some(party) = party_option {
        html!(
            <div class="h-10 w-full border-b border-slate-400 bg-slate-700 flex justify-center pointer-events-auto relative" >
                <div class="p-2 absolute left-0">
                    {"Floor "}{party.current_floor}{", room "}{party.rooms_explored.on_current_floor}
                    {": "}
                    {format!("{}", party.current_room.room_type)}
                </div>
                if battle_option.is_some() { <TurnOrderBar /> }
                    else { <RoomExplorationTracker /> }
            </div>
        )
    } else {
        html!(<div>{"Error - no party found"}</div>)
    }
}

#[function_component(RoomExplorationTracker)]
fn room_exploration_tracker() -> Html {
    let (game_state, _) = use_store::<GameStore>();
    let party_option = get_current_party_option(&game_state);
    if let Some(party) = party_option {
        html!(
        <ul class="h-full list-none flex items-center" >
            {party.client_current_floor_rooms_list
                .iter()
                .enumerate()
                .map(|(i, room_type_option )| {
                    let current_room_class = if party.rooms_explored.on_current_floor == ( i + 1 ) as u16 {
                        "border border-yellow-400"
                    } else {
                        "border-slate-400"
                    };

                    let connection_line = if i != party.client_current_floor_rooms_list.len() - 1 {
                        html!(<span
                              class={format!("h-[2px] bg-slate-400" )}
                              style={format!("width: {SPACING_REM}rem;")}
                              />)
                    } else {
                        html!()
                    };

                    html!(
                        <>
                            <li class={ format!("pr-2 pl-2 border text-sm flex items-center justify-center {}", current_room_class) }
                                style={format!("height: {BUTTON_HEIGHT_SMALL}rem")}
                            >
                                {match room_type_option {
                                    Some(room_type) => {format!("{room_type}")},
                                    None => { "?".to_string() },
                                }}
                            </li>
                            {connection_line}
                        </>
                        )
                })
                .collect::<Html>()}
        </ul>
        )
    } else {
        html!()
    }
}
