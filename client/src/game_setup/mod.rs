use crate::{
    game_setup::{
        character_and_party_selection::CharacterAndPartySelection, game_setup_menu::GameSetupMenu,
    },
    lobby::user_list::UserList,
};
pub mod character_and_party_selection;
pub mod game_setup_menu;
use leptos::*;

#[component]
pub fn game_setup() -> impl IntoView {
    view! {
        <main class="h-screen w-screen p-4 bg-teal-950 text-zinc-300 flex flex-col">
            <GameSetupMenu/>
            <div class="w-full flex flex-1">
                <CharacterAndPartySelection/>
                <UserList/>
            </div>
        </main>
    }
}
