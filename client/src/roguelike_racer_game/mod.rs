pub mod dungeon_room;
use crate::roguelike_racer_game::dungeon_room::DungeonRoom;
use common::adventuring_party::AdventuringParty;
use leptos::*;

#[component]
pub fn roguelike_racer_game(cx: Scope) -> impl IntoView {
    view! { cx,
    <main class="h-screen w-screen flex flex-wrap bg-green-200" >
        <div class="bg-teal-950 h-1/2 w-full block">
            <DungeonRoom />
        </div>
        // <div class="bg-green-200 p-4">"lmao"</div>
        <div class="h-1/2 w-full flex">
            <div class="bg-yellow-200 p-4 w-1/2 lg:w-auto flex-1">
            </div>
            <div class="bg-pink-200 p-4 w-1/2 lg:w-[61.8%]">"mlao"</div>
        </div>
    </main>
    }
}
