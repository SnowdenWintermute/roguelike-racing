pub mod send_client_input;
use common::{errors::AppError, packets::server_to_client::GameServerUpdatePackets};
use gloo::console::log;
use wasm_bindgen::prelude::Closure;
use wasm_bindgen::JsCast;
use web_sys::{MessageEvent, WebSocket};
use yew::prelude::*;
use yewdux::prelude::use_store;

use crate::{
    components::alerts::set_alert,
    store::{
        alert_store::AlertStore, game_store::GameStore, lobby_store::LobbyStore,
        websocket_store::WebsocketStore,
    },
};

#[derive(Properties, PartialEq)]
pub struct Props {
    // pub children: Html,
    pub server_url: String,
}

#[derive(Default, Clone)]
pub struct CustomFormData {}

#[function_component(WebsocketManager)]
pub fn websocket_manager(props: &Props) -> Html {
    let (_, websocket_dispatch) = use_store::<WebsocketStore>();
    let (_, lobby_dispatch) = use_store::<LobbyStore>();
    let (_, game_dispatch) = use_store::<GameStore>();
    let (alert_state, alert_dispatch) = use_store::<AlertStore>();
    let server_url = props.server_url.clone();

    use_effect_with((), move |_| {
        let websocket = WebSocket::new(&server_url);
        match websocket {
            Ok(websocket_success) => {
                // For small binary messages, like CBOR, Arraybuffer is more efficient than Blob handling
                websocket_success.set_binary_type(web_sys::BinaryType::Arraybuffer);
                let cloned_ws = websocket_success.clone();
                let onmessage_callback = Closure::<dyn FnMut(_)>::new(move |e: MessageEvent| {
                    let result = (|| -> Result<(), AppError> {
                        if let Ok(abuf) = e.data().dyn_into::<js_sys::ArrayBuffer>() {
                            let array = js_sys::Uint8Array::new(&abuf);
                            let byte_slice = &array.to_vec()[..];
                            let deserialized: Result<GameServerUpdatePackets, _> =
                                serde_cbor::from_slice(byte_slice);
                            if let Ok(data) = deserialized {
                                match data {
                                    GameServerUpdatePackets::Error(message) => {
                                        let dispatch = alert_dispatch.clone();
                                        let cloned_alert_state = alert_state.clone();
                                        set_alert(cloned_alert_state, dispatch, message);
                                    }
                                    GameServerUpdatePackets::FullUpdate(update) => {
                                        let dispatch = lobby_dispatch.clone();
                                        dispatch.reduce_mut(|store| {
                                            store.game_list = update.game_list.games;
                                            store.room = update.room;
                                        });
                                    }
                                    GameServerUpdatePackets::RoomFullUpdate(update) => {
                                        // room.update(move |room_state| *room_state = update)
                                    }
                                    GameServerUpdatePackets::UserLeftRoom(username_leaving) => {
                                        // handle_user_left_room(room, &username_leaving)
                                    }
                                    GameServerUpdatePackets::UserJoinedRoom(update) => {
                                        // room.update(move |room_state| room_state.users.push(update))
                                    }
                                    GameServerUpdatePackets::GameList(update) => {
                                        let dispatch = lobby_dispatch.clone();
                                        dispatch.reduce_mut(|store| store.game_list = update.games);
                                    }
                                    GameServerUpdatePackets::GameFullUpdate(update) => {
                                        let dispatch = game_dispatch.clone();
                                        dispatch.reduce_mut(|store| {
                                            store.game = update;
                                        });
                                    }
                                    GameServerUpdatePackets::UserJoinedGame(update) => {
                                        // handle_user_joined_game(game, update)
                                    }
                                    GameServerUpdatePackets::UserLeftGame(username) => {
                                        // handle_user_left_game(game, username)
                                    }
                                    GameServerUpdatePackets::AdventuringPartyCreated(
                                        party_creation,
                                    ) => {
                                        // handle_adventuring_party_created(game, party_creation)?,
                                    }
                                    GameServerUpdatePackets::AdventuringPartyRemoved(party_id) => {
                                        // handle_adventuring_party_removed(game, party_id)
                                    }
                                    GameServerUpdatePackets::ClientAdventuringPartyId(update) => {
                                        // party_id.update(move |id| {
                                        //     id.0 = update;
                                        // })
                                    }
                                    GameServerUpdatePackets::PlayerChangedAdventuringParty(
                                        update,
                                    ) => {

                                        // handle_player_changed_adventuring_party(game, update),
                                    }
                                    GameServerUpdatePackets::CharacterCreation(
                                        character_in_party,
                                    ) => {
                                        // handle_character_creation(game, character_in_party)?
                                    }
                                    _ => {
                                        log!(format!("unhandled packet: {:#?}", data))
                                    }
                                }
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
                            log!("unhandled error");
                            // alerts::set_alert(
                            //     alerts,
                            //     app_error.message.clone(),
                            //     &mut last_alert_id,
                            // );
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
