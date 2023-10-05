use common::game::player_actions::PlayerInputs;
use common::packets::server_to_client::{ClientGameListState, GameListEntry};
use leptos::{logging::log, *};
use web_sys::WebSocket;

use crate::common_components::button_basic::ButtonBasic;
use crate::websocket_provider::send_client_input::send_client_input;

#[component]
pub fn game_list() -> impl IntoView {
    let game_list_state = expect_context::<RwSignal<ClientGameListState>>();
    let game_list = move || game_list_state().games;

    create_effect(move |_| log!("game list: {:#?}", game_list()));

    view! {
        <section class="flex-1 p-4 mr-4 bg-slate-700 border border-slate-400" id="game_list" >
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

    view! {
        <li class="w-full flex border border-slate-400 mb-4 justify-between">
            <div class="flex">
                <div class="h-10 flex items-center w-20 border-r border-slate-400 pl-4" >
                    <div class="overflow-hidden whitespace-nowrap overflow-ellipsis">
                        {game_name.clone()}
                    </div>
                </div>
                <div class="h-10 flex items-center w-24 border-r border-slate-400 pl-4" >
                    <div class="overflow-hidden whitespace-nowrap overflow-ellipsis">
                        "Players:" {number_of_users}
                    </div>
                </div>
            </div>
            <ButtonBasic
                on:click=move |_| send_client_input(ws, PlayerInputs::JoinGame(game_name.clone()))
                extra_styles="border-r-0 border-t-0 border-b-0"
                disabled={MaybeSignal::derive(move || time_started.is_some())}
            >
                "Join"
            </ButtonBasic>
        </li>
    }
}
