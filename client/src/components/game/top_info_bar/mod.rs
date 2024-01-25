use crate::components::game::turn_order_bar::TurnOrderBar;
use crate::store::game_store::get_current_battle_option;
use crate::store::game_store::get_current_party_option;
use crate::store::game_store::GameStore;
use gloo::console::log;
use yew::prelude::*;
use yewdux::prelude::use_store;

#[function_component(TopInfoBar)]
pub fn game() -> Html {
    let (game_state, _) = use_store::<GameStore>();
    let battle_option = get_current_battle_option(&game_state);
    let party_option = get_current_party_option(&game_state);

    let top_bar_display_state = use_state(|| 1);
    let cloned_top_bar_display_state = top_bar_display_state.clone();
    let on_click_top_bar = Callback::from(move |_| {
        if *cloned_top_bar_display_state == 1 {
            cloned_top_bar_display_state.set(2)
        } else if *cloned_top_bar_display_state == 2 {
            cloned_top_bar_display_state.set(0)
        } else {
            cloned_top_bar_display_state.set(1)
        }
    });

    let top_bar_display_class = match *top_bar_display_state {
        2 => "top-0 left-0",
        0 => "bottom-0 right-0 ",
        _ => "top-0 left-1/2 -translate-x-1/2",
    };

    html!(
    <button
        onclick={on_click_top_bar}
        class={ format!( "absolute z-40 bg-slate-700 {top_bar_display_class}" )}
    >
    <div class="border-b border-r border-l border-slate-400 bg-slate-700 w-full text-center " >
        if battle_option.is_some() {
                <TurnOrderBar />
            } else if let Some(party) = party_option {
                <ul class="w-full list-none flex justify-center border-b border-slate-400" >
                    {party.client_current_floor_rooms_list
                        .iter()
                        .enumerate()
                        .map(|(i, room_type_option )| {
                            log!(format!("room box index: {i} explored on current floor: {}", party.rooms_explored.on_current_floor));
                            let current_room_class = if party.rooms_explored.on_current_floor == ( i + 1 ) as u16 {
                                "border-yellow-400"
                            } else {
                                "border-slate-400"
                            };

                            html!(
                                <li class={ format!("p-2 border {}", current_room_class) }>
                                {match room_type_option {
                                    Some(room_type) => {format!("{room_type}")},
                                    None => { "?".to_string() },
                                }}
                                </li>
                                )
                        })
                        .collect::<Html>()}
                </ul>
                <div class="p-2" >
                    {"Floor "}{party.current_floor}{", room "}{party.rooms_explored.on_current_floor}
                    {": "}
                    {format!("{}", party.current_room.room_type)}
                </div>
            } else {
                <div>{"Error - no party found"}</div>
            }
    </div>
    </button>
    )
}
