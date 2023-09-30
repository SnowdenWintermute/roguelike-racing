use common::adventuring_party::AdventuringParty;
use common::game::player_actions::{PlayerInputRequest, PlayerInputs};

use leptos::*;
use web_sys::WebSocket;

#[component]
pub fn dungeon_room(cx: Scope) -> impl IntoView {
    let adventuring_party = expect_context::<RwSignal<Option<AdventuringParty>>>(cx);
    let ws = expect_context::<ReadSignal<Option<WebSocket>>>(cx);

    // let current_floor: Memo<u8> = create_memo(cx, move |_| {
    //     adventuring_party.with(|adventuring_party| adventuring_party.get().expec.current_floor)
    // });

    let send_test_bytes = move |_| {
        ws.with(|socket| match socket {
            Some(ws) => {
                let some_player_action = PlayerInputs::SelectConsumable(0);
                let serialized = serde_cbor::to_vec(&some_player_action);

                match serialized {
                    Ok(bytes) => ws.send_with_u8_array(bytes.as_slice()),
                    Err(_) => Ok(()),
                };
            }
            None => {
                println!("no websocket in global state");
                ()
            }
        });
        ()
    };

    view! { cx,
    <section class="h-full w-full p-2 text-zinc-300" >
        <div class="bg-slate-700 p-2 h-full">
        <ul class="list-none">
            <li>
                <button on:click=send_test_bytes>"Send"</button>
                // "adventuring party id: "{id}
            </li>
            <li>
                // "current floor: "{move || current_floor.get()}
            </li>
        </ul>
        </div>
    </section> }
}
