use crate::{
    game_setup::{
        character_and_party_selection::CharacterAndPartySelection, game_setup_menu::GameSetupMenu,
    },
    lobby::user_list::UserList,
};
pub mod adventuring_party_lobby_card;
pub mod character_and_party_selection;
pub mod game_setup_menu;
use common::game::RoguelikeRacerGame;
use leptos::*;

#[component]
pub fn game_setup() -> impl IntoView {
    // let (is_client, set_is_client) = create_signal(false);
    // create_effect(move |_| set_is_client(true));

    view! {
        <main class="h-screen w-screen p-4 bg-teal-950 text-zinc-300 flex flex-col">
            // <Show when=move || is_client()
            // fallback=move || view! {""}>
            <GameSetupMenu/>
            <div class="w-full flex flex-1">
                <CharacterAndPartySelection />
                <UserList/>
            </div>
            // </Show>
        </main>
    }
}
