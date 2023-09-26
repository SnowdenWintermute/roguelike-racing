use crate::home_page::AppState;
use common::{
    adventuring_party::AdventuringParty,
    game::player_actions::{PlayerInputRequest, PlayerInputs},
};
use leptos::*;
use wasm_bindgen::JsValue;

#[component]
pub fn dungeon_room(cx: Scope) -> impl IntoView {
    // let adventuring_party = expect_context::<RwSignal<AdventuringParty>>(cx);
    let app_state = expect_context::<RwSignal<AppState>>(cx);
    // this is horrible
    let current_floor: Memo<u8> = create_memo(cx, move |_| {
        app_state.with(move |state| {
            state
                .adventuring_party
                .with(|adventuring_party| adventuring_party.current_floor)
        })
    });

    let send_test_bytes = move |_| {
        app_state.with(move |state| {
            state.ws.with(|socket| match socket {
                Some(ws) => {
                    let some_player_action = PlayerInputRequest {
                        party_id: 0,
                        player_character_id: 0,
                        player_input: PlayerInputs::SelectConsumable(0),
                    };
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
        })
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
                "current floor: "{move || current_floor.get()}
            </li>
        </ul>
        </div>
    </section> }
}
