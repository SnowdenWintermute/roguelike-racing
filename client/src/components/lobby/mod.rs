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
        <main class="min-h-screen max-h-screen w-screen bg-slate-800 text-zinc-300 justify-center overflow-y-auto">
            <div class="p-4 h-screen max-h-screen max-w-[80rem] mx-auto flex flex-col" >
                <LobbyMenu />
                <div class="flex flex-grow" >
                    <div class="flex flex-col flex-grow" >
                        <WelcomeInfo />
                        <GameList/>
                    </div>
                    <UserList />
                </div>
            </div>
        </main>
    )
}
