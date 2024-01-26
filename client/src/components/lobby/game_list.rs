use crate::components::lobby::game_list_item::GameListItem;
use crate::store::lobby_store::LobbyStore;
use yew::prelude::*;
use yewdux::prelude::use_store;

#[function_component(GameList)]
pub fn game_list() -> Html {
    let (lobby_state, _) = use_store::<LobbyStore>();

    html!(
        <section class="flex-1 p-4 mr-4 bg-slate-700 border border-slate-400 overflow-y-auto" id="game_list">
            <h2 class="text-slate-200 text-l mb-2">{ "Games" }</h2>
            <ul class="list-none">
                {lobby_state.game_list.iter().map(|game|
                    html!{<GameListItem game={game.clone()} />}).collect::<Html>()}
            </ul>
        </section>
    )
}
