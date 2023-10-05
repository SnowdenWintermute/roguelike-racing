#![allow(dead_code, unused_imports)]
use actix::prelude::*;
use common::consts::MAIN_CHAT_ROOM;
use common::game::player_actions::PlayerInputs;
use common::game::RoguelikeRacerGame;
use common::utils::generate_random_username;
use rand::{self, rngs::ThreadRng, Rng};
use std::collections::{HashMap, HashSet};
use std::sync::atomic::AtomicUsize;
use std::sync::Arc;
pub mod connection_handler;
pub mod disconnection_handler;
pub mod list_rooms_handler;
pub mod player_input_handlers;
pub mod send_messages;
pub mod update_packet_creators;
use super::{AppMessage, ClientBinaryMessage, ClientMessage};
use crate::websocket_server::game_server::player_input_handlers::create_game_handler::create_game_handler;
use crate::websocket_server::game_server::player_input_handlers::game_list_update_request_handler::game_list_update_request_handler;
use crate::websocket_server::game_server::player_input_handlers::join_game_handler::join_game_handler;
use crate::websocket_server::game_server::player_input_handlers::leave_game_handler::leave_game_handler;

#[derive(Debug)]
pub struct ConnectedUser {
    pub id: usize,
    pub actor_address: Recipient<AppMessage>,
    pub username: String,
    pub current_room_name: String,
    pub current_game_name: Option<String>,
}

impl ConnectedUser {
    pub fn new(id: usize, actor_address: Recipient<AppMessage>) -> Self {
        ConnectedUser {
            id,
            actor_address,
            username: generate_random_username(),
            current_room_name: MAIN_CHAT_ROOM.to_string(),
            current_game_name: None,
        }
    }
}

#[derive(Debug)]
pub struct GameServer {
    sessions: HashMap<usize, ConnectedUser>,
    rooms: HashMap<String, HashSet<usize>>,
    games: HashMap<String, RoguelikeRacerGame>,
    visitor_count: Arc<AtomicUsize>,
}

impl GameServer {
    pub fn new(visitor_count: Arc<AtomicUsize>) -> GameServer {
        let mut rooms = HashMap::new();
        rooms.insert(MAIN_CHAT_ROOM.to_owned(), HashSet::new());

        GameServer {
            sessions: HashMap::new(),
            rooms,
            games: HashMap::new(),
            visitor_count,
        }
    }
}

impl Actor for GameServer {
    type Context = Context<Self>;
}

impl Handler<ClientBinaryMessage> for GameServer {
    type Result = ();
    fn handle(&mut self, message: ClientBinaryMessage, _: &mut Context<Self>) {
        println!("message received: {:?}", message.content);
        let byte_slice = &message.content[..];
        let deserialized: Result<PlayerInputs, _> = serde_cbor::from_slice(byte_slice);
        match deserialized {
            Ok(PlayerInputs::CreateGame(game_creation_data)) => {
                create_game_handler(self, message.actor_id, game_creation_data)
            }
            Ok(PlayerInputs::JoinGame(game_name)) => {
                join_game_handler(self, message.actor_id, game_name)
            }
            Ok(PlayerInputs::LeaveGame) => leave_game_handler(self, message.actor_id),
            Ok(PlayerInputs::RequestGameList) => {
                game_list_update_request_handler(self, message.actor_id)
            }
            _ => println! {"unhandled binary message\n {:#?}:",deserialized},
        }
    }
}

impl Handler<ClientMessage> for GameServer {
    type Result = ();
    fn handle(&mut self, message: ClientMessage, _: &mut Context<Self>) {
        println!("message received: {}", message.content);
        let room = &self
            .sessions
            .get(&message.actor_id)
            .expect("if we got a message from this id, the user should exist in our list")
            .current_room_name;
        self.send_string_message(&room, message.content.as_str());
    }
}
