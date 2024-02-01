pub mod adventuring_party_lobby_card;
pub mod character_and_party_selection;
pub mod character_creation_menu;
pub mod character_lobby_card;
pub mod game_setup_menu;
use yew::prelude::*;

use crate::components::lobby::game_setup::character_and_party_selection::CharacterAndPartySelection;
use crate::components::lobby::game_setup::game_setup_menu::GameSetupMenu;
use crate::components::lobby::user_list::UserList;

#[function_component(GameSetup)]
pub fn game_setup() -> Html {
    html!(
        <main class="min-h-screen w-screen bg-slate-800 flex justify-center">
            <div class="w-full max-w-[80rem] p-4 text-zinc-300 flex flex-col" >
                <GameSetupMenu/>
                <div class="w-full flex flex-1">
                    <CharacterAndPartySelection />
                    <UserList/>
                </div>
            </div>
        </main>
    )
}
