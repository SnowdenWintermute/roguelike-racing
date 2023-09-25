use common::adventuring_party::AdventuringParty;
use leptos::*;

use crate::dungeon_room::DungeonRoom;

#[component]
pub fn home_page(cx: Scope) -> impl IntoView {
    let adventuring_party = provide_context(cx, create_rw_signal(cx, AdventuringParty::new(0)));

    view! { cx, <main class="h-screen w-screen flex flex-wrap bg-green-200" >
        <div class="bg-teal-950 h-1/2 w-full block">
            <DungeonRoom />
        </div>
        // <div class="bg-green-200 p-4">"lmao"</div>
        <div class="h-1/2 w-full flex">
            <div class="bg-yellow-200 p-4 w-1/2 lg:w-auto flex-1">
                "User Input Buttons"
            </div>
            <div class="bg-pink-200 p-4 w-1/2 lg:w-[61.8%]">"mlao"</div>
        </div>
    </main> }
}
