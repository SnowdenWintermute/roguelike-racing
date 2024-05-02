use crate::yew_app::{
    components::lobby::game_list_item::GameListItem, store::lobby_store::LobbyStore,
};
use yew::prelude::*;
use yewdux::prelude::use_store;

#[function_component(GameList)]
pub fn game_list() -> Html {
    let (lobby_state, _) = use_store::<LobbyStore>();

    html!(
        <section class="p-4 mr-4 bg-slate-700 border border-slate-400 overflow-y-auto flex-grow relative" id="game_list pointer-events-auto">

                // <div class="h-[80%] absolute top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2" >
                //     <img src="public/img/logo.svg" class="h-full filter" />
                // </div>
            <h2 class="text-slate-200 text-l mb-2">{ "Games" }</h2>
            <ul class="list-none">
                {lobby_state.game_list.iter().map(|game|
                    html!{<GameListItem game={game.clone()} />}).collect::<Html>()}
            </ul>
        </section>
    )
}
