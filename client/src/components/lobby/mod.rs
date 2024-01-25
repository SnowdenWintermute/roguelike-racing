pub mod game_list;
pub mod game_list_item;
pub mod game_setup;
pub mod lobby_menu;
pub mod user_list;
mod welcome_info;
use crate::components::lobby::game_list::GameList;
use crate::components::lobby::lobby_menu::LobbyMenu;
use crate::components::lobby::user_list::UserList;
use crate::components::lobby::welcome_info::WelcomeInfo;
use yew::prelude::*;

#[function_component(Lobby)]
pub fn lobby() -> Html {
    html!(
        <main class="min-h-screen w-screen p-4 bg-slate-800 text-zinc-300 flex flex-col">
            <LobbyMenu/>
            <div class="w-full flex flex-1" >
                <div class="w-full flex flex-col" >
                    <WelcomeInfo />
                    <GameList/>
                </div>
                <UserList/>
            </div>
        </main>
    )
}
