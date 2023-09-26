use crate::dungeon_room::DungeonRoom;
use common::adventuring_party::AdventuringParty;
use common::game::player_actions::PlayerInputRequest;
use common::game::player_actions::PlayerInputs;
use leptos::*;
use web_sys::{ErrorEvent, MessageEvent, WebSocket};

#[derive(Clone, Debug)]
pub struct AppState {
    pub ws: ReadSignal<Option<WebSocket>>,
    pub adventuring_party: ReadSignal<AdventuringParty>,
}

#[component]
pub fn home_page(cx: Scope) -> impl IntoView {
    let (ws, set_ws) = create_signal::<Option<WebSocket>>(cx, None);
    let (adventuring_party, set_adventuring_party) =
        create_signal::<AdventuringParty>(cx, AdventuringParty::new(0));

    let app_state = provide_context(
        cx,
        create_rw_signal(
            cx,
            AppState {
                ws,
                adventuring_party,
            },
        ),
    );

    // connect the socket and put it in a signal
    create_effect(cx, move |_| {
        let websocket = WebSocket::new("ws://127.0.0.1:8080/ws");
        match websocket {
            Ok(websocket_success) => set_ws(Some(websocket_success)),
            _ => println!("websocket failed to create"),
        }
    });

    // signal can be updated
    create_effect(cx, move |_| {
        set_adventuring_party.update(|party| party.current_floor = 5)
    });

    view! { cx, <main class="h-screen w-screen flex flex-wrap bg-green-200" >
        <div class="bg-teal-950 h-1/2 w-full block">
            <DungeonRoom />
        </div>
        // <div class="bg-green-200 p-4">"lmao"</div>
        <div class="h-1/2 w-full flex">
            <div class="bg-yellow-200 p-4 w-1/2 lg:w-auto flex-1">
            </div>
            <div class="bg-pink-200 p-4 w-1/2 lg:w-[61.8%]">"mlao"</div>
        </div>
    </main> }
}
