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
    let (game_state, game_dispatch) = use_store::<GameStore>();
    let (websocket_state, _) = use_store::<WebsocketStore>();
    let (lobby_state, _) = use_store::<LobbyStore>();

    let game = game_state
        .game
        .clone()
        .expect("component only shown if game exists");
    let player = game
        .players
        .get(&lobby_state.username)
        .expect("a player should exist by the username stored on the client")
        .clone();

    let party_id = game_state.current_party_id.expect("must have party id");

    let party = game
        .adventuring_parties
        .get(&party_id)
        .expect("must have a party id")
        .clone();

    let players_ready_to_descend_option =
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
    // let cloned_websocket_state = websocket_state.clone();
    // let handle_click_descend = Callback::from(move |_| {
    //     send_client_input(
    //         &cloned_websocket_state.websocket,
    //         PlayerInputs::ToggleReadyToGoDownStairs,
    //         )
    // });

    let ready_to_explore_buttons = party
        .player_usernames
        .iter()
        .map(|item| {
            let is_ready = party.players_ready_to_explore.contains(item);
            let is_player_of_this_client = lobby_state.username == *item;
            let conditional_classes = if !is_ready {
                "opacity-50"
            } else {
                ""
            };
            let cloned_handle_click_explore = handle_click_explore.clone();
            let handle_click = if is_player_of_this_client {
                Callback::from(move |e|{
                    cloned_handle_click_explore.emit(e)
                })
            } else {
                Callback::from(move|_|())
            };

            html!(
                <li>
                    <button 
                        onclick={handle_click}
                        class={format!("border border-slate-400 bg-slate-700 h-10 pr-2 pl-2 {conditional_classes}
                                       pointer-events-auto mr-2 last:mr-0")}
                        >
                          {item}
                      </button>
                </li>
                )
        })
        .collect::<Html>();

    html!(
    <div class="flex">
        <span class="border border-slate-400 bg-slate-700 h-10 pr-2 pl-2 mr-4 flex justify-center items-center pointer-events-auto">
            {"Players ready to explore: "}
        </span>
        <ul>
            {ready_to_explore_buttons}
        </ul>
    </div>
    )
}
