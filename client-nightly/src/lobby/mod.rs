use crate::lobby::game_list::GameList;
use crate::lobby::lobby_menu::LobbyMenu;
pub mod game_list;
pub mod lobby_menu;
use leptos::logging::log;
use leptos::*;

#[component]
pub fn lobby() -> impl IntoView {
    let (is_client, set_is_client) = create_signal(false);
    create_effect(move |_| set_is_client(true));
    create_effect(move |_| log!("is client: {}", is_client()));

    view! {
    <main class="h-screen w-screen p-4 bg-teal-950 text-zinc-300 flex flex-col" >
        <LobbyMenu />
        <GameList />
    </main>
    }
}

