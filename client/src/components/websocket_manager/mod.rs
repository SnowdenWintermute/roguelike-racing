mod adventuring_party_update_handlers;
mod game_full_update_handler;
mod handle_battle_victory_report;
mod handle_character_dropped_item;
mod handle_character_picked_up_item;
pub mod handle_combat_turn_results;
mod handle_packet;
mod in_game_party_update_handlers;
mod inventory_management_update_handlers;
mod lobby_update_handlers;
pub mod send_client_input;
mod websocket_channel_packet_handlers;
use crate::components::alerts::set_alert;
use crate::store::alert_store::AlertStore;
use crate::store::game_store::GameStore;
use crate::store::lobby_store::LobbyStore;
use crate::store::websocket_store::WebsocketStore;
use common::errors::AppError;
use common::packets::server_to_client::GameServerUpdatePackets;
use gloo::console::log;
use wasm_bindgen::prelude::Closure;
use wasm_bindgen::JsCast;
use web_sys::MessageEvent;
use web_sys::WebSocket;
use yew::prelude::*;
use yewdux::prelude::use_store;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub server_url: String,
}

#[derive(Default, Clone)]
pub struct CustomFormData {}

#[function_component(WebsocketManager)]
pub fn websocket_manager(props: &Props) -> Html {
    let (_, websocket_dispatch) = use_store::<WebsocketStore>();
    let (lobby_state, lobby_dispatch) = use_store::<LobbyStore>();
    let (_, game_dispatch) = use_store::<GameStore>();
    let (_, alert_dispatch) = use_store::<AlertStore>();
    let server_url = props.server_url.clone();

    use_effect_with((), move |_| {
        let websocket = WebSocket::new(&server_url);
        match websocket {
            Ok(websocket_success) => {
                // For small binary messages, like CBOR, Arraybuffer is more efficient than Blob handling
                websocket_success.set_binary_type(web_sys::BinaryType::Arraybuffer);
                let cloned_ws = websocket_success.clone();
                let cloned_websocket_dispatch = websocket_dispatch.clone();
                let onmessage_callback = Closure::<dyn FnMut(_)>::new(move |e: MessageEvent| {
                    let result = (|| -> Result<(), AppError> {
                        if let Ok(abuf) = e.data().dyn_into::<js_sys::ArrayBuffer>() {
                            let array = js_sys::Uint8Array::new(&abuf);
                            let byte_slice = &array.to_vec()[..];
                            let deserialized: Result<GameServerUpdatePackets, _> =
                                serde_cbor::from_slice(byte_slice);
                            if let Ok(data) = deserialized {
                                let cloned_alert_dispatch = alert_dispatch.clone();
                                let cloned_lobby_dispatch = lobby_dispatch.clone();
                                let cloned_lobby_state = lobby_state.clone();
                                let cloned_game_dispatch = game_dispatch.clone();
                                let cloned_websocket_dispatch = cloned_websocket_dispatch.clone();

                                handle_packet::handle_packet(
                                    data,
                                    cloned_alert_dispatch,
                                    cloned_lobby_dispatch,
                                    cloned_lobby_state,
                                    cloned_game_dispatch,
                                    cloned_websocket_dispatch,
                                )?
                            }
                        } else if let Ok(txt) = e.data().dyn_into::<js_sys::JsString>() {
                            log!("message event, received Text: {:?}", txt);
                        } else {
                            log!("message event, received Unknown: {:?}", e.data());
                        }
                        Ok(())
                    })();
                    match result {
                        Err(app_error) => {
                            log!(format!("app error: {}", app_error.message));
                            let dispatch = alert_dispatch.clone();
                            set_alert(dispatch, app_error.message);
                        }
                        Ok(()) => (),
                    };
                });
                cloned_ws.set_onmessage(Some(onmessage_callback.as_ref().unchecked_ref()));
                onmessage_callback.forget(); // forget the callback to keep it alive
                                             // websocket_state.set(Some(websocket_success));
                let dispatch = websocket_dispatch.clone();
                dispatch.reduce_mut(|store| store.websocket = Some(websocket_success));
            }
            _ => println!("websocket failed to create"),
        }
    });

    html!(
        <div id="websocket-manager">
        </div>
    )
}
