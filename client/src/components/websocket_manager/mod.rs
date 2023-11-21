pub mod adventuring_party_update_handlers;
mod inventory_management_update_handlers;
pub mod lobby_update_handlers;
pub mod send_client_input;
use self::lobby_update_handlers::handle_user_left_game;
use crate::components::alerts::set_alert;
use crate::components::websocket_manager::inventory_management_update_handlers::handle_character_equipped_item;
use crate::components::websocket_manager::{
    adventuring_party_update_handlers::{
        handle_adventuring_party_created, handle_character_creation, handle_character_deletion,
        handle_player_changed_adventuring_party,
    },
    lobby_update_handlers::{
        handle_game_started, handle_player_toggled_ready, handle_user_joined_game,
        handle_user_left_room,
    },
};
use crate::store::websocket_store::WebsocketStore;
use crate::store::{alert_store::AlertStore, game_store::GameStore, lobby_store::LobbyStore};
use common::{errors::AppError, packets::server_to_client::GameServerUpdatePackets};
use gloo::console::log;
use wasm_bindgen::prelude::Closure;
use wasm_bindgen::JsCast;
use web_sys::{MessageEvent, WebSocket};
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
                                // log!(format!("{:#?}", data));
                                match data {
                                    GameServerUpdatePackets::Error(message) => {
                                        let dispatch = alert_dispatch.clone();
                                        set_alert(dispatch, message);
                                    }
                                    GameServerUpdatePackets::ClientUserName(username) => {
                                        lobby_dispatch.clone().reduce_mut(|store| {
                                            store.username = username;
                                        });
                                    }
                                    GameServerUpdatePackets::FullUpdate(update) => {
                                        lobby_dispatch.clone().reduce_mut(|store| {
                                            store.game_list = update.game_list.games;
                                            store.room = update.room;
                                        });
                                    }
                                    GameServerUpdatePackets::RoomFullUpdate(update) => {
                                        lobby_dispatch.clone().reduce_mut(|store| {
                                            store.room = update;
                                        });
                                    }
                                    GameServerUpdatePackets::UserLeftRoom(username_leaving) => {
                                        lobby_dispatch.clone().reduce_mut(|store| {
                                            handle_user_left_room(store, &username_leaving)
                                        })
                                    }
                                    GameServerUpdatePackets::UserJoinedRoom(update) => {
                                        lobby_dispatch.clone().reduce_mut(|store| {
                                            store.room.users.push(update);
                                        })
                                    }
                                    GameServerUpdatePackets::GameList(update) => {
                                        let dispatch = lobby_dispatch.clone();
                                        dispatch.reduce_mut(|store| store.game_list = update.games);
                                    }
                                    GameServerUpdatePackets::GameFullUpdate(update) => {
                                        game_dispatch.clone().reduce_mut(|store| {
                                            store.game = update;
                                        });
                                    }
                                    GameServerUpdatePackets::UserJoinedGame(username) => {
                                        game_dispatch.clone().reduce_mut(|store| {
                                            handle_user_joined_game(store, username)
                                        })
                                    }
                                    GameServerUpdatePackets::UserLeftGame(username) => {
                                        game_dispatch.clone().reduce_mut(|store| {
                                            handle_user_left_game(store, username)
                                        })
                                    }
                                    GameServerUpdatePackets::AdventuringPartyCreated(
                                        party_creation,
                                    ) => game_dispatch.clone().reduce_mut(|store| {
                                        handle_adventuring_party_created(store, party_creation)
                                    }),
                                    GameServerUpdatePackets::ClientAdventuringPartyId(update) => {
                                        game_dispatch.clone().reduce_mut(|store| {
                                            store.current_party_id = update;
                                        })
                                    }
                                    GameServerUpdatePackets::PlayerChangedAdventuringParty(
                                        update,
                                    ) => game_dispatch.clone().reduce_mut(|store| {
                                        handle_player_changed_adventuring_party(store, update)
                                    }),
                                    GameServerUpdatePackets::CharacterCreation(
                                        character_in_party,
                                    ) => game_dispatch.clone().reduce_mut(|store| {
                                        let _ =
                                            handle_character_creation(store, character_in_party);
                                    }),
                                    GameServerUpdatePackets::CharacterDeletion(
                                        character_deletion,
                                    ) => game_dispatch.clone().reduce_mut(|store| {
                                        let _ =
                                            handle_character_deletion(store, character_deletion);
                                    }),
                                    GameServerUpdatePackets::PlayerToggledReady(username) => {
                                        game_dispatch.clone().reduce_mut(|store| {
                                            let _ = handle_player_toggled_ready(store, username);
                                        })
                                    }
                                    GameServerUpdatePackets::GameStarted(timestamp) => {
                                        game_dispatch.clone().reduce_mut(move |store| {
                                            handle_game_started(store, timestamp)
                                        })
                                    }
                                    GameServerUpdatePackets::CharacterEquippedItem(packet) => {
                                        game_dispatch.clone().reduce_mut(|store| {
                                            let _ = handle_character_equipped_item(store, packet);
                                        })
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
