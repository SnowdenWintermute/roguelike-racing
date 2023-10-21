pub mod game_setup_menu;
use yew::prelude::*;
use yewdux::prelude::use_store;

use crate::components::lobby::game_setup::game_setup_menu::GameSetupMenu;

#[function_component(GameSetup)]
pub fn game_setup() -> Html {
    // let (lobby_state, _) = use_store::<LobbyStore>();

    html!(
        <main class="min-h-screen w-screen p-4 bg-teal-950 text-zinc-300 flex flex-col">
            <GameSetupMenu/>
            <div class="w-full flex flex-1">
                // <CharacterAndPartySelection/>
                // <UserList/>
            </div>
        </main>
    )
}
