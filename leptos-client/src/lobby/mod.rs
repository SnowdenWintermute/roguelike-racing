pub mod game_list;
pub mod lobby_menu;
pub mod user_list;
use crate::lobby::game_list::GameList;
use crate::lobby::lobby_menu::LobbyMenu;
use crate::lobby::user_list::UserList;
use leptos::*;

#[component]
pub fn lobby() -> impl IntoView {
    view! {
        <main class="min-h-screen w-screen p-4 bg-teal-950 text-zinc-300 flex flex-col">
            <LobbyMenu/>
            <div class="w-full flex flex-1">
                <GameList/>
                <UserList/>
            </div>
        </main>
    }
}
