pub mod send_client_input;
pub mod server_update_handlers;
use self::server_update_handlers::adventuring_party_lobby_update_handlers::handle_adventuring_party_created;
use self::server_update_handlers::adventuring_party_lobby_update_handlers::handle_adventuring_party_removed;
use self::server_update_handlers::adventuring_party_lobby_update_handlers::handle_player_changed_adventuring_party;
use crate::alerts;
use crate::alerts::Alert;
use crate::home_page::ClientPartyId;
use crate::websocket_provider::server_update_handlers::adventuring_party_lobby_update_handlers::handle_character_creation;
use crate::websocket_provider::server_update_handlers::lobby_update_handlers::handle_user_joined_game;
use crate::websocket_provider::server_update_handlers::lobby_update_handlers::handle_user_left_game;
use crate::websocket_provider::server_update_handlers::lobby_update_handlers::handle_user_left_room;
use common::adventuring_party::AdventuringParty;
use common::errors::AppError;
use common::game::RoguelikeRacerGame;
use common::packets::server_to_client::ClientGameListState;
use common::packets::server_to_client::GameServerUpdatePackets;
use common::packets::server_to_client::RoomState;
use leptos::logging::log;
use leptos::*;
use wasm_bindgen::prelude::Closure;
use wasm_bindgen::JsCast;
use web_sys::{MessageEvent, WebSocket};

#[component]
pub fn websocket_provider(children: Children) -> impl IntoView {
    let (ws, set_ws) = create_signal::<Option<WebSocket>>(None);
    provide_context(ws);
    let _adventuring_party = expect_context::<RwSignal<Option<AdventuringParty>>>();
    let game_list = expect_context::<RwSignal<ClientGameListState>>();
    let room = expect_context::<RwSignal<RoomState>>();
    let game = expect_context::<RwSignal<Option<RoguelikeRacerGame>>>();
    let party_id = expect_context::<RwSignal<ClientPartyId>>();
    let alerts = expect_context::<RwSignal<Vec<Alert>>>();
    let mut last_alert_id: u32 = 0;

    create_effect(move |_| {
        let websocket = WebSocket::new("ws://127.0.0.1:8080/ws");
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
                                        alerts::set_alert(
                                            alerts,
                                            message.clone(),
                                            &mut last_alert_id,
                                        );
                                    }
                                    GameServerUpdatePackets::FullUpdate(update) => {
                                        game_list.update(move |game_list_state| {
                                            *game_list_state = update.game_list
                                        });
                                        room.update(move |room_state| *room_state = update.room)
                                    }
                                    GameServerUpdatePackets::GameList(update) => game_list
                                        .update(move |game_list_state| *game_list_state = update),
                                    GameServerUpdatePackets::RoomFullUpdate(update) => {
                                        room.update(move |room_state| *room_state = update)
                                    }
                                    GameServerUpdatePackets::UserLeftRoom(username_leaving) => {
                                        handle_user_left_room(room, &username_leaving)
                                    }
                                    GameServerUpdatePackets::UserJoinedRoom(update) => {
                                        room.update(move |room_state| room_state.users.push(update))
                                    }
                                    GameServerUpdatePackets::UserJoinedGame(update) => {
                                        handle_user_joined_game(game, update)
                                    }
                                    GameServerUpdatePackets::UserLeftGame(username) => {
                                        handle_user_left_game(game, username)
                                    }
                                    GameServerUpdatePackets::GameFullUpdate(update) => {
                                        game.update(move |game_state| *game_state = update)
                                    }
                                    GameServerUpdatePackets::AdventuringPartyCreated(
                                        party_creation,
                                    ) => handle_adventuring_party_created(game, party_creation)?,
                                    GameServerUpdatePackets::AdventuringPartyRemoved(party_id) => {
                                        handle_adventuring_party_removed(game, party_id)
                                    }
                                    GameServerUpdatePackets::ClientAdventuringPartyId(update) => {
                                        party_id.update(move |id| {
                                            id.0 = update;
                                        })
                                    }
                                    GameServerUpdatePackets::PlayerChangedAdventuringParty(
                                        update,
                                    ) => handle_player_changed_adventuring_party(game, update),
                                    GameServerUpdatePackets::CharacterCreation(
                                        character_in_party,
                                    ) => handle_character_creation(game, character_in_party)?,
                                    _ => log!("unhandled game server packet: {:#?}", data),
                                };
                            } else {
                                println!("error deserializing")
                            };
                        } else if let Ok(txt) = e.data().dyn_into::<js_sys::JsString>() {
                            log!("message event, received Text: {:?}", txt);
                        } else {
                            log!("message event, received Unknown: {:?}", e.data());
                        }
                        Ok(())
                    })();
                    match result {
                        Err(app_error) => {
                            alerts::set_alert(
                                alerts,
                                app_error.message.clone(),
                                &mut last_alert_id,
                            );
                        }
                        Ok(()) => (),
                    };
                });

                cloned_ws.set_onmessage(Some(onmessage_callback.as_ref().unchecked_ref()));
                onmessage_callback.forget(); // forget the callback to keep it alive
                set_ws(Some(websocket_success));
            }
            _ => println!("websocket failed to create"),
        }
    });

    view! { <div aria-hidden=true>{children()}</div> }
}
