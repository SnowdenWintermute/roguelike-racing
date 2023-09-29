use crate::dungeon_room::DungeonRoom;
use crate::lobby::Lobby;
use common::adventuring_party::AdventuringParty;
use common::game::player_actions::PlayerInputRequest;
use common::game::player_actions::PlayerInputs;
use common::game::RoguelikeRacerGame;
use common::packets::server_to_client::GameServerUpdatePackets;
use leptos::*;
use leptos_use::use_websocket;
use wasm_bindgen::prelude::Closure;
use wasm_bindgen::JsCast;
use web_sys::{ErrorEvent, MessageEvent, WebSocket};

#[component]
pub fn home_page(cx: Scope) -> impl IntoView {
    let (ws, set_ws) = create_signal::<Option<WebSocket>>(cx, None);
    provide_context(cx, create_rw_signal(cx, AdventuringParty::new(0)));
    provide_context(cx, create_rw_signal::<Option<RoguelikeRacerGame>>(cx, None));
    let adventuring_party = expect_context::<RwSignal<AdventuringParty>>(cx);

    provide_context(cx, ws);
    // connect the socket and put it in a signal
    create_effect(cx, move |_| {
        let websocket = WebSocket::new("ws://127.0.0.1:8080/ws");
        match websocket {
            Ok(websocket_success) => {
                // For small binary messages, like CBOR, Arraybuffer is more efficient than Blob handling
                websocket_success.set_binary_type(web_sys::BinaryType::Arraybuffer);
                let cloned_ws = websocket_success.clone();
                let onmessage_callback = Closure::<dyn FnMut(_)>::new(move |e: MessageEvent| {
                    // Handle difference Text/Binary,...
                    if let Ok(abuf) = e.data().dyn_into::<js_sys::ArrayBuffer>() {
                        log!("message event, received arraybuffer: {:?}", abuf);
                        let array = js_sys::Uint8Array::new(&abuf);
                        let len = array.byte_length() as usize;
                        log!("Arraybuffer received {}bytes: {:?}", len, array.to_vec());
                        //
                        let byte_slice = &array.to_vec()[..];
                        let deserialized: Result<GameServerUpdatePackets, _> =
                            serde_cbor::from_slice(byte_slice);
                        log!("line41: {:#?}", deserialized);
                    } else if let Ok(txt) = e.data().dyn_into::<js_sys::JsString>() {
                        log!("message event, received Text: {:?}", txt);
                    } else {
                        log!("message event, received Unknown: {:?}", e.data());
                    }
                });

                websocket_success.set_onmessage(Some(onmessage_callback.as_ref().unchecked_ref()));
                onmessage_callback.forget(); // forget the callback to keep it alive
                set_ws(Some(websocket_success));
            }
            _ => println!("websocket failed to create"),
        }
    });

    // signal can be updated
    create_effect(cx, move |_| {
        adventuring_party.update(|party| party.current_floor += 1);
    });

    view! { cx, <main class="h-screen w-screen flex flex-wrap bg-green-200" >
        <div class="bg-teal-950 h-1/2 w-full block">
            <DungeonRoom />
        </div>
        // <div class="bg-green-200 p-4">"lmao"</div>
        <div class="h-1/2 w-full flex">
            <div class="bg-yellow-200 p-4 w-1/2 lg:w-auto flex-1">
                <Lobby />
            </div>
            <div class="bg-pink-200 p-4 w-1/2 lg:w-[61.8%]">"mlao"</div>
        </div>
    </main> }
}
