mod adventuring_party_update_handlers;
mod battle_full_update_handler;
mod character_selected_combat_action_handler;
mod dungeon_floor_number_changed_handler;
mod game_full_update_handler;
pub mod handle_battle_victory_report;
mod handle_character_dropped_equipped_item;
mod handle_character_dropped_item;
mod handle_character_picked_up_item;
pub mod handle_combat_turn_results;
mod handle_packet;
mod handle_raw_action_results;
mod in_game_party_update_handlers;
mod inventory_management_update_handlers;
mod lobby_update_handlers;
mod new_dungeon_room_handler;
mod new_game_message_handler;
pub mod send_client_input;
mod websocket_channel_packet_handlers;
use crate::yew_app::components::alerts::set_alert;
use crate::yew_app::store::alert_store::AlertStore;
use crate::yew_app::store::bevy_communication_store::BevyCommunicationStore;
use crate::yew_app::store::game_store::GameStore;
use crate::yew_app::store::lobby_store::LobbyStore;
use crate::yew_app::store::websocket_store::WebsocketStore;
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
    let (_, lobby_dispatch) = use_store::<LobbyStore>();
    let (_, game_dispatch) = use_store::<GameStore>();
    let (_, alert_dispatch) = use_store::<AlertStore>();
    let (_, bevy_communication_dispatch) = use_store::<BevyCommunicationStore>();
    let server_url = props.server_url.clone();
    // log!(format!(
    //     "attempting connection to websocket server: {}",
    //     props.server_url
    // ));

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
                                let cloned_game_dispatch = game_dispatch.clone();
                                let cloned_websocket_dispatch = cloned_websocket_dispatch.clone();
                                let cloned_bevy_communication_dispatch =
                                    bevy_communication_dispatch.clone();
                                handle_packet::handle_packet(
                                    data,
                                    cloned_alert_dispatch,
                                    cloned_lobby_dispatch,
                                    cloned_game_dispatch,
                                    cloned_websocket_dispatch,
                                    cloned_bevy_communication_dispatch,
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
            _ => log!("websocket failed to create"),
        }
    });

    html!(
        <div id="websocket-manager">
        </div>
    )
}
