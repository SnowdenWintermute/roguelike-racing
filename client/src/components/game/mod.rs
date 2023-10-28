pub mod action_menu;
pub mod combat_log;
pub mod dungeon_room;
use yew::prelude::*;
use yewdux::prelude::use_store;

use crate::{
    components::game::{action_menu::ActionMenu, combat_log::CombatLog, dungeon_room::DungeonRoom},
    store::game_store::GameStore,
};

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
                <CombatLog />
            </div>
        </main>
    )
}
