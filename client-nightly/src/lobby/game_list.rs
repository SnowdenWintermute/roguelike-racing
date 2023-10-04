use common::game::player_actions::PlayerInputs;
use common::packets::server_to_client::{ClientGameListState, GameListEntry};
use leptos::{logging::log, *};
use web_sys::{MouseEvent, WebSocket};

use crate::websocket_provider::send_client_input::send_client_input;

#[component]
pub fn game_list() -> impl IntoView {
    let _ws = expect_context::<ReadSignal<Option<WebSocket>>>();
    let game_list_state = expect_context::<RwSignal<ClientGameListState>>();
    let game_list = move || game_list_state().games;

    create_effect(move |_| log!("game list: {:#?}", game_list()));

    view! {
        <section id="game_list" class="flex-1 p-4 bg-slate-700 border border-lime-500">
            <h3>"Games"</h3>
            <ul class="list-none">
                  <For
                each=game_list
                key=|game| game.game_name.clone()
                children=move |game| {
                  view! {
                   <GamesListElement game=game/>
                  }
            }
              />
            </ul>
        </section>
    }
}

#[component]
fn games_list_element(game: GameListEntry) -> impl IntoView {
    let ws = expect_context::<ReadSignal<Option<WebSocket>>>();
    let GameListEntry {
        game_name,
        number_of_users,
        time_started,
    } = game;

    let join_game =
        move |e: MouseEvent| send_client_input(ws, PlayerInputs::JoinGame(event_target_value(&e)));

    view! {
        <li class="h-10 w-full flex border border-lime-500 p-4 mb-4">

            <button class="" value=game_name.clone() on:click=join_game>"Join Game " {game_name.clone()} </button>
        </li>
    }
}
