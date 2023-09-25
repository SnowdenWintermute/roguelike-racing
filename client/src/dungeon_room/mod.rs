use common::adventuring_party::AdventuringParty;
use leptos::*;

#[component]
pub fn dungeon_room(cx: Scope) -> impl IntoView {
    let adventuring_party = expect_context::<RwSignal<AdventuringParty>>(cx);
    let current_floor = create_memo(cx, move |_| {
        adventuring_party.with(|adventuring_party| adventuring_party.current_floor)
    });
    let id = create_memo(cx, move |_| {
        adventuring_party.with(|adventuring_party| adventuring_party.id)
    });

    view! { cx,
    <section class="h-full w-full p-2 text-zinc-300" >
        <div class="bg-slate-700 p-2 h-full">
        <ul class="list-none">
            <li>
                "adventuring party id: "{id}
            </li>
            <li>
                "current floor: "{current_floor}
            </li>
        </ul>
        </div>
    </section> }
}
