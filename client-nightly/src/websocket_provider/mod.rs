pub mod send_client_input;
use common::adventuring_party::AdventuringParty;
use common::packets::server_to_client::ClientGameListState;
use common::packets::server_to_client::GameServerUpdatePackets;
use common::packets::server_to_client::RoguelikeRacerAppState;
use leptos::logging::log;
use leptos::*;
use wasm_bindgen::prelude::Closure;
use wasm_bindgen::JsCast;
use web_sys::{MessageEvent, WebSocket};

#[component]
pub fn websocket_provider(children: Children) -> impl IntoView {
    let (ws, set_ws) = create_signal::<Option<WebSocket>>(None);
    provide_context(ws);
    let _app_state = expect_context::<RwSignal<RoguelikeRacerAppState>>();
    let _adventuring_party = expect_context::<RwSignal<Option<AdventuringParty>>>();
    let game_list = expect_context::<RwSignal<ClientGameListState>>();

    create_effect(move |_| {
        let websocket = WebSocket::new("ws://127.0.0.1:8080/ws");
        match websocket {
            Ok(websocket_success) => {
                // For small binary messages, like CBOR, Arraybuffer is more efficient than Blob handling
                websocket_success.set_binary_type(web_sys::BinaryType::Arraybuffer);
                let cloned_ws = websocket_success.clone();
                let onmessage_callback = Closure::<dyn FnMut(_)>::new(move |e: MessageEvent| {
                    if let Ok(abuf) = e.data().dyn_into::<js_sys::ArrayBuffer>() {
                        let array = js_sys::Uint8Array::new(&abuf);
                        let byte_slice = &array.to_vec()[..];
                        let deserialized: Result<GameServerUpdatePackets, _> =
                            serde_cbor::from_slice(byte_slice);
                        log!("line41: {:#?}", deserialized);
                        if let Ok(data) = deserialized {
                            match data {
                                GameServerUpdatePackets::FullUpdate(update) => {
                                    game_list.update(move |game_list_state| {
                                        *game_list_state = update.game_list.clone()
                                    })
                                }
                                GameServerUpdatePackets::GameList(update) => {
                                    log!("got game list: {:#?}", update);
                                    game_list.update(move |game_list_state| {
                                        *game_list_state = update.clone()
                                    })
                                }
                            };
                        };
                    } else if let Ok(txt) = e.data().dyn_into::<js_sys::JsString>() {
                        log!("message event, received Text: {:?}", txt);
                    } else {
                        log!("message event, received Unknown: {:?}", e.data());
                    }
                });

                cloned_ws.set_onmessage(Some(onmessage_callback.as_ref().unchecked_ref()));
                onmessage_callback.forget(); // forget the callback to keep it alive
                set_ws(Some(websocket_success));
            }
            _ => println!("websocket failed to create"),
        }
    });

    view! { <div aria-hidden=true>{children()}</div>}
}
