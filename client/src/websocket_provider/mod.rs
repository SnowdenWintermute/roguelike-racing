use common::game::player_actions::PlayerInputRequest;
use common::game::player_actions::PlayerInputs;
use common::packets::server_to_client::GameServerUpdatePackets;
use leptos::*;
use wasm_bindgen::prelude::Closure;
use wasm_bindgen::JsCast;
use web_sys::{ErrorEvent, MessageEvent, WebSocket};

#[component]
pub fn websocket_provider(cx: Scope, children: Children) -> impl IntoView {
    let (ws, set_ws) = create_signal::<Option<WebSocket>>(cx, None);
    provide_context(cx, ws);

    create_effect(cx, move |_| {
        let websocket = WebSocket::new("ws://127.0.0.1:8080/ws");
        match websocket {
            Ok(websocket_success) => {
                // For small binary messages, like CBOR, Arraybuffer is more efficient than Blob handling
                websocket_success.set_binary_type(web_sys::BinaryType::Arraybuffer);
                let cloned_ws = websocket_success.clone();
                let onmessage_callback = Closure::<dyn FnMut(_)>::new(move |e: MessageEvent| {
                    if let Ok(abuf) = e.data().dyn_into::<js_sys::ArrayBuffer>() {
                        let array = js_sys::Uint8Array::new(&abuf);
                        let len = array.byte_length() as usize;
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

    view! { cx, <div aria-hidden=true>{children(cx)}</div>}
}
