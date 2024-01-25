pub mod game_list;
pub mod game_list_item;
pub mod game_setup;
pub mod lobby_menu;
pub mod user_list;
use crate::components::lobby::game_list::GameList;
use crate::components::lobby::lobby_menu::LobbyMenu;
use crate::components::lobby::user_list::UserList;
use yew::prelude::*;

#[function_component(Lobby)]
pub fn lobby() -> Html {
    html!(
        <main class="min-h-screen w-screen p-4 bg-slate-800 text-zinc-300 flex flex-col">
            <LobbyMenu/>
            <div class="w-full flex flex-1">
                <GameList/>
                <UserList/>
            </div>
        </main>
    )
}
