#![allow(dead_code, unused_imports)]
use actix::prelude::*;
use common::game::player_actions::PlayerInputs;
use common::game::RoguelikeRacerGame;
use rand::{self, rngs::ThreadRng, Rng};
use std::collections::{HashMap, HashSet};
use std::sync::atomic::AtomicUsize;
use std::sync::Arc;
pub mod connection_handler;
pub mod disconnection_handler;
pub mod join_room_handler;
pub mod list_rooms_handler;
pub mod player_input_handler;
pub mod send_messages;

use crate::websocket_server::game_server::player_input_handler::create_game_handler::create_game_handler;

use super::{
    AppMessage, ClientBinaryMessage, ClientMessage, Disconnect, Join, ListRooms, MessageContent,
    MAIN_CHAT_ROOM,
};

#[derive(Debug)]
pub struct ConnectedUser {
    pub id: usize,
    pub actor_address: Recipient<AppMessage>,
    pub username: Option<String>,
    pub current_room_name: String,
    pub current_game_name: Option<String>,
}

impl ConnectedUser {
    pub fn new(id: usize, actor_address: Recipient<AppMessage>) -> Self {
        ConnectedUser {
            id,
            actor_address,
            username: None,
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
        // deserialize and handle message
        let byte_slice = &message.content[..];
        let deserialized: Result<PlayerInputs, _> = serde_cbor::from_slice(byte_slice);
        match deserialized {
            Ok(PlayerInputs::CreateGame(game_creation_data)) => {
                create_game_handler(self, game_creation_data)
            }
            _ => {
                println! {"unhandled binary message\n {:#?}:",deserialized}
            }
        }

        let room = &self
            .sessions
            .get(&message.actor_id)
            .expect("if we got a message from this id, the user should exist in our list")
            .current_room_name;
        // right now all we do is send it to everyone in the same room with this function:
        self.send_byte_message(&room, &message.content.clone());
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
