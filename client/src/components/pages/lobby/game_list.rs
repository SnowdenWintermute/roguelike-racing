use common::packets::server_to_client::GameListEntry;
use yew::prelude::*;
use yewdux::prelude::use_store;

use crate::store::lobby_store::LobbyStore;

#[function_component(GameList)]
pub fn game_list() -> Html {
    let (lobby_state, _) = use_store::<LobbyStore>();

    html!(
        <section class="flex-1 p-4 mr-4 bg-slate-700 border border-slate-400" id="game_list">
            <h2 class="text-slate-200 text-l mb-2">{ "Games" }</h2>
            <ul class="list-none">
                {lobby_state.game_list.iter().map(|game|
                    html!{<GameListItem game={game.clone()} />}).collect::<Html>()}
            </ul>
        </section>

    )
}

#[derive(Properties, PartialEq)]
pub struct GameListItemProps {
    pub game: GameListEntry,
}

#[function_component(GameListItem)]
pub fn game_list_item(props: &GameListItemProps) -> Html {
    html!(
        <li class="w-full flex border border-slate-400 mb-4 justify-between">
            <div class="flex">
                <div class="h-10 flex items-center w-20 border-r border-slate-400 pl-4">
                    <div class="overflow-hidden whitespace-nowrap overflow-ellipsis">
                        {props.game.game_name.clone()}
                    </div>
                </div>
                <div class="h-10 flex items-center w-24 border-r border-slate-400 pl-4">
                    <div class="overflow-hidden whitespace-nowrap overflow-ellipsis">
                        {"Players:"} {props.game.number_of_users}
                    </div>
                </div>
            </div>
            // <ButtonBasic
            //     on:click=move |_| send_client_input(ws, PlayerInputs::JoinGame(game_name.clone()))
            //     extra_styles="border-r-0 border-t-0 border-b-0"
            //     disabled=MaybeSignal::derive(move || time_started.is_some())
            // >
            //     "Join"
            // </ButtonBasic>
        </li>
    )
}
