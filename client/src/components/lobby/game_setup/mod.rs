pub mod adventuring_party_lobby_card;
pub mod character_and_party_selection;
pub mod game_setup_menu;
use yew::prelude::*;

use crate::components::lobby::{
    game_setup::{
        character_and_party_selection::CharacterAndPartySelection, game_setup_menu::GameSetupMenu,
    },
    user_list::UserList,
};

#[function_component(GameSetup)]
pub fn game_setup() -> Html {
    html!(
        <main class="min-h-screen w-screen p-4 bg-teal-950 text-zinc-300 flex flex-col">
            <GameSetupMenu/>
            <div class="w-full flex flex-1">
                <CharacterAndPartySelection />
                <UserList/>
            </div>
        </main>
    )
}
