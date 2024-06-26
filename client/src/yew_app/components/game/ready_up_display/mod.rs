use std::collections::HashSet;
use std::rc::Rc;
use crate::yew_app::components::websocket_manager::send_client_input::send_client_input;
use crate::yew_app::store::game_store::GameStore;
use crate::yew_app::store::lobby_store::LobbyStore;
use crate::yew_app::store::websocket_store::WebsocketStore;
use common::dungeon_rooms::DungeonRoomTypes;
use common::packets::client_to_server::PlayerInputs;
use yew::prelude::*;
use yewdux::use_store;

#[function_component(ReadyUpDisplay)]
pub fn ready_up_display() -> Html {
    let (game_state, _) = use_store::<GameStore>();
    let (websocket_state, _) = use_store::<WebsocketStore>();
    let (lobby_state, _) = use_store::<LobbyStore>();

    let game = game_state
        .game
        .clone()
        .expect("component only shown if game exists");

    let party_id = game_state.current_party_id.expect("must have party id");

    let party = game
        .adventuring_parties
        .get(&party_id)
        .expect("must have a party id")
        .clone();

    let _players_ready_to_descend_option =
        if party.current_room.room_type == DungeonRoomTypes::Stairs {
            Some(party.players_ready_to_descend.clone())
        } else {
            None
        };

    if party.battle_id.is_some() {
        return html!();
    };

    let cloned_websocket_state = websocket_state.clone();
    let handle_click_explore = Callback::from(move |_| {
        send_client_input(
            &cloned_websocket_state.websocket,
            PlayerInputs::ToggleReadyToExplore,
            )
    });
    let cloned_websocket_state = websocket_state.clone();
    let handle_click_descend = Callback::from(move |_| {
        send_client_input(
            &cloned_websocket_state.websocket,
            PlayerInputs::ToggleReadyToGoDownStairs,
            )
    });

    let ready_to_explore_buttons =
        create_ready_buttons(&party.player_usernames, &party.players_ready_to_explore, &lobby_state, handle_click_explore);
    let ready_to_descend_buttons =
        create_ready_buttons(&party.player_usernames, &party.players_ready_to_descend, &lobby_state, handle_click_descend);

    html!(
        <div class="max-w-fit" id="ready-to-explore-display">
            <div class="border border-slate-400 bg-slate-700 h-10 pr-2 pl-2 mr-4 mb-1 pointer-events-auto flex items-center max-w-fit">
            if party.current_room.room_type != DungeonRoomTypes::Stairs {
                {"Players ready to explore: "}
            } else {
                {"Players voting to stay on current floor: "}
            }
            </div>
            <ul class="flex mb-2">
                {ready_to_explore_buttons}
            </ul>
            if party.current_room.room_type == DungeonRoomTypes::Stairs {
                <div class="border border-slate-400 bg-slate-700 h-10 pr-2 pl-2 mr-4 mb-1 pointer-events-auto flex items-center">
                    {"Players voting to descend deeper into the dungeon: "}
                </div>
                <ul class="flex">
                    {ready_to_descend_buttons}
                </ul>
            }
        </div>
    )
}

fn create_ready_buttons(usernames: &HashSet<String>,list_of_ready_users: &HashSet<String>, lobby_state: &Rc<LobbyStore>, click_handler: Callback<MouseEvent> ) -> Html {

    usernames
        .iter()
        .map(|item| {
            let is_ready = list_of_ready_users.contains(item);
            let is_player_of_this_client = lobby_state.username == *item;
            let conditional_classes = if !is_ready {
                "opacity-50"
            } else {
                ""
            };
            let cloned_handle_click_explore = click_handler.clone();
            let handle_click = if is_player_of_this_client {
                Callback::from(move |e|{
                    cloned_handle_click_explore.emit(e)
                })
            } else {
                Callback::from(move|_|())
            };

            html!(
                <li class="mr-2 last:mr-0">
                    <button 
                        onclick={handle_click}
                        class={format!("border border-slate-400 bg-slate-700 h-10 pr-2 pl-2 {conditional_classes}
                                       pointer-events-auto ")}
                        >
                          {item}
                      </button>
                </li>
                )
        })
        .collect::<Html>()
}
