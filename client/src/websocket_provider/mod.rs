pub mod send_client_input;
use common::adventuring_party::AdventuringParty;
use common::game::RoguelikeRacerGame;
use common::game::RoguelikeRacerPlayer;
use common::packets::server_to_client::ClientGameListState;
use common::packets::server_to_client::GameServerUpdatePackets;
use common::packets::server_to_client::PlayerAdventuringPartyChange;
use common::packets::server_to_client::RoomState;
use leptos::logging::log;
use leptos::*;
use wasm_bindgen::prelude::Closure;
use wasm_bindgen::JsCast;
use web_sys::{MessageEvent, WebSocket};

use crate::home_page::ClientPartyId;

#[component]
pub fn websocket_provider(children: Children) -> impl IntoView {
    let (ws, set_ws) = create_signal::<Option<WebSocket>>(None);
    provide_context(ws);
    let _adventuring_party = expect_context::<RwSignal<Option<AdventuringParty>>>();
    let game_list = expect_context::<RwSignal<ClientGameListState>>();
    let room = expect_context::<RwSignal<RoomState>>();
    let game = expect_context::<RwSignal<Option<RoguelikeRacerGame>>>();
    let party_id = expect_context::<RwSignal<ClientPartyId>>();

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
                                        *game_list_state = update.game_list
                                    });
                                    room.update(move |room_state| *room_state = update.room)
                                }
                                GameServerUpdatePackets::GameList(update) => {
                                    log!("got game list: {:#?}", update);
                                    game_list
                                        .update(move |game_list_state| *game_list_state = update)
                                }
                                GameServerUpdatePackets::RoomFullUpdate(update) => {
                                    log!("got room update: {:#?}", update);
                                    room.update(move |room_state| *room_state = update)
                                }
                                GameServerUpdatePackets::UserLeftRoom(update) => {
                                    log!("user left room: {:#?}", update);
                                    room.update(move |room_state| {
                                        for (index, username) in
                                            room_state.users.clone().iter().enumerate()
                                        {
                                            if &update == username {
                                                room_state.users.remove(index);
                                            }
                                        }
                                    })
                                }
                                GameServerUpdatePackets::UserJoinedRoom(update) => {
                                    log!("user joined room: {:#?}", update);
                                    room.update(move |room_state| room_state.users.push(update))
                                }
                                GameServerUpdatePackets::UserJoinedGame(update) => {
                                    game.update(move |game_option| {
                                        if let Some(game) = game_option {
                                            game.partyless_players.insert(
                                                update.clone(),
                                                RoguelikeRacerPlayer::new(None, update),
                                            );
                                        }
                                    })
                                }
                                GameServerUpdatePackets::UserLeftGame(username) => {
                                    game.update(move |game_option| {
                                        if let Some(game) = game_option {
                                            game.partyless_players.remove(&username.clone());
                                            game.remove_player_from_adventuring_party(
                                                username.clone(),
                                            );
                                        }
                                    })
                                }
                                GameServerUpdatePackets::GameFullUpdate(update) => {
                                    log!("received full game update: {:#?}", update);
                                    game.update(move |game_state| *game_state = update)
                                }
                                GameServerUpdatePackets::AdventuringPartyCreated(update) => game
                                    .update(move |game_state| {
                                        if let Some(game) = game_state {
                                            println!("adventuring party created in current game");
                                            game.adventuring_parties.insert(update.id, update);
                                        }
                                    }),
                                GameServerUpdatePackets::AdventuringPartyRemoved(update) => game
                                    .update(move |game_state| {
                                        if let Some(game) = game_state {
                                            println!("adventuring party removed from current game");
                                            game.adventuring_parties.remove(&update);
                                        };
                                    }),
                                GameServerUpdatePackets::ClientAdventuringPartyId(update) => {
                                    party_id.update(move |id| {
                                        id.0 = update;
                                    })
                                }
                                GameServerUpdatePackets::PlayerChangedAdventuringParty(update) => {
                                    game.update(move |game_state| {
                                        println!(
                                            "adventuring party change requested: {:#?}",
                                            update
                                        )
                                    })
                                }

                                _ => log!("unknown binary packet"),
                            };
                        } else {
                            println!("error deserializing")
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

    view! { <div aria-hidden=true>{children()}</div> }
}
