use crate::dungeon_room::DungeonRoom;
use common::adventuring_party::AdventuringParty;
use common::game::player_actions::PlayerInputRequest;
use common::game::player_actions::PlayerInputs;
use leptos::*;
use leptos_use::use_websocket;
use wasm_bindgen::prelude::Closure;
use wasm_bindgen::JsCast;
use web_sys::{ErrorEvent, MessageEvent, WebSocket};

// #[derive(Clone, Debug)]
// pub struct AppState {
//     pub ws: ReadSignal<Option<WebSocket>>,
//     pub adventuring_party: ReadSignal<AdventuringParty>,
// }

#[component]
pub fn home_page(cx: Scope) -> impl IntoView {
    let (ws, set_ws) = create_signal::<Option<WebSocket>>(cx, None);
    provide_context(cx, create_rw_signal(cx, AdventuringParty::new(0)));
    let adventuring_party = expect_context::<RwSignal<AdventuringParty>>(cx);

    provide_context(cx, ws);
    // connect the socket and put it in a signal
    create_effect(cx, move |_| {
        let websocket = WebSocket::new("ws://127.0.0.1:8080/ws");
        match websocket {
            Ok(websocket_success) => {
                let cloned_ws = websocket_success.clone();
                let onmessage_callback = Closure::<dyn FnMut(_)>::new(move |e: MessageEvent| {
                    // Handle difference Text/Binary,...
                    if let Ok(abuf) = e.data().dyn_into::<js_sys::ArrayBuffer>() {
                        log!("message event, received arraybuffer: {:?}", abuf);
                        let array = js_sys::Uint8Array::new(&abuf);
                        let len = array.byte_length() as usize;
                        log!("Arraybuffer received {}bytes: {:?}", len, array.to_vec());
                        // here you can for example use Serde Deserialize decode the message
                        // for demo purposes we switch back to Blob-type and send off another binary message
                        cloned_ws.set_binary_type(web_sys::BinaryType::Blob);
                        match cloned_ws.send_with_u8_array(&[5, 6, 7, 8]) {
                            Ok(_) => log!("binary message successfully sent"),
                            Err(err) => log!("error sending message: {:?}", err),
                        }
                    } else if let Ok(blob) = e.data().dyn_into::<web_sys::Blob>() {
                        log!("message event, received blob: {:?}", blob);
                        // better alternative to juggling with FileReader is to use https://crates.io/crates/gloo-file
                        let fr = web_sys::FileReader::new().unwrap();
                        let fr_c = fr.clone();
                        // create onLoadEnd callback
                        let onloadend_cb =
                            Closure::<dyn FnMut(_)>::new(move |_e: web_sys::ProgressEvent| {
                                let array = js_sys::Uint8Array::new(&fr_c.result().unwrap());
                                let len = array.byte_length() as usize;
                                log!("Blob received {}bytes: {:?}", len, array.to_vec());
                                // here you can for example use the received image/png data
                            });
                        fr.set_onloadend(Some(onloadend_cb.as_ref().unchecked_ref()));
                        fr.read_as_array_buffer(&blob).expect("blob not readable");
                        onloadend_cb.forget();
                    } else if let Ok(txt) = e.data().dyn_into::<js_sys::JsString>() {
                        log!("message event, received Text: {:?}", txt);
                    } else {
                        log!("message event, received Unknown: {:?}", e.data());
                    }
                });
                // set message event handler on WebSocket
                websocket_success.set_onmessage(Some(onmessage_callback.as_ref().unchecked_ref()));
                // forget the callback to keep it alive
                onmessage_callback.forget();
                websocket_success.set_binary_type(web_sys::BinaryType::Arraybuffer);
                // websocket_success.set_onmessage(Some());
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
            </div>
            <div class="bg-pink-200 p-4 w-1/2 lg:w-[61.8%]">"mlao"</div>
        </div>
    </main> }
}
