use crate::lobby::game_list::GameList;
use crate::lobby::list_rendering::DynamicList;
use crate::lobby::list_rendering_example::DynamicListExample;
use crate::lobby::lobby_menu::LobbyMenu;
use crate::websocket_provider::send_client_input::send_client_input;
pub mod game_list;
pub mod list_rendering;
pub mod list_rendering_example;
pub mod lobby_menu;
use common::adventuring_party::AdventuringParty;
use common::game::player_actions::{GameCreation, PlayerInputRequest, PlayerInputs};
use common::game::RoguelikeRacerGame;
use common::packets::server_to_client::RoguelikeRacerAppState;
use leptos::ev::InputEvent;
use leptos::*;
use web_sys::WebSocket;

#[component]
pub fn lobby(cx: Scope) -> impl IntoView {
    let (is_client, set_is_client) = create_signal(cx, false);
    create_effect(cx, move |_| set_is_client.set(true));
    create_effect(cx, move |_| log!("is client: {}", is_client.get()));

    view! { cx,
    <main class="h-screen w-screen p-4 bg-teal-950 text-zinc-300 flex flex-col" >
        // <DynamicList initial_length=1 />
        // <DynamicListExample initial_length=1 />
        <LobbyMenu />
        <Show
            when=move || is_client.get()
            fallback=|cx| view! {cx, <div/>}>
            <GameList />
        </Show>
    </main>
    }
}
