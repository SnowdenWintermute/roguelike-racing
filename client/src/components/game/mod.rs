pub mod action_menu;
pub mod combat_log;
pub mod combatant_detail_tab;
pub mod dungeon_room;
pub mod tabbed_display;
use crate::{
    components::game::{
        action_menu::ActionMenu, dungeon_room::DungeonRoom, tabbed_display::TabbedDisplay,
    },
    store::game_store::GameStore,
};
use yew::prelude::*;
use yewdux::prelude::use_store;

#[function_component(Game)]
pub fn game() -> Html {
    let (game_state, _) = use_store::<GameStore>();
    let game = game_state
        .game
        .clone()
        .expect("component only shown if game exists");

    html!(
        <main class="h-screen w-screen p-4 bg-gray-600 text-zinc-300 flex flex-col">
            <DungeonRoom game={game} party_id={game_state.current_party_id.expect("must have party id")} />
            <div class="flex h-1/2 max-h-[453px]" >
                <ActionMenu />
                <TabbedDisplay />
            </div>
        </main>
    )
}
