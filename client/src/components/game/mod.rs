pub mod action_menu;
pub mod combat_log;
pub mod dungeon_room;
use yew::prelude::*;

use crate::components::game::{
    action_menu::ActionMenu, combat_log::CombatLog, dungeon_room::DungeonRoom,
};

#[function_component(Game)]
pub fn game() -> Html {
    html!(
        <main class="min-h-screen w-screen p-4 bg-gray-600 text-zinc-300 flex flex-col">
            <DungeonRoom />
            <div class="flex flex-1 max-h-[453px]" >
                <ActionMenu />
                <CombatLog />
            </div>
        </main>
    )
}
