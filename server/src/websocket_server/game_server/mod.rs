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
pub mod join_server_handler;
pub mod message_handler;

use super::{
    AppMessage, ClientBinaryMessage, ClientMessage, Disconnect, Join, ListRooms, MessageContent,
    MAIN_CHAT_ROOM,
};

#[derive(Debug)]
pub struct GameServer {
    sessions: HashMap<usize, Recipient<AppMessage>>,
    rooms: HashMap<String, HashSet<usize>>,
    rng: ThreadRng,
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
            rng: rand::thread_rng(),
            games: HashMap::new(),
            visitor_count,
        }
    }
}

impl Actor for GameServer {
    /// We are going to use simple Context, we just need ability to communicate
    /// with other actors.
    type Context = Context<Self>;
}

impl GameServer {
    fn send_message(&self, room: &str, message: &str, skip_id: usize) {
        if let Some(sessions) = self.rooms.get(room) {
            for id in sessions {
                if *id == skip_id {
                    continue;
                }

                if let Some(session_address) = self.sessions.get(id) {
                    session_address.do_send(AppMessage(MessageContent::Str(message.to_owned())));
                }
            }
        }
    }

    fn send_byte_message(&self, room: &str, message: &Vec<u8>, skip_id: usize) {
        if let Some(sessions) = self.rooms.get(room) {
            for id in sessions {
                // if *id == skip_id {
                //     continue;
                // }
                // self.games
                if let Some(addr) = self.sessions.get(id) {
                    addr.do_send(AppMessage(MessageContent::Bytes(message.to_vec())));
                }
            }
        }
    }
}

impl Handler<ClientMessage> for GameServer {
    type Result = ();
    fn handle(&mut self, message: ClientMessage, _: &mut Context<Self>) {
        println!("message received: {}", message.content);
        self.send_message(&message.room, message.content.as_str(), message.sender_id);
    }
}

impl Handler<ClientBinaryMessage> for GameServer {
    type Result = ();
    fn handle(&mut self, message: ClientBinaryMessage, _: &mut Context<Self>) {
        println!("message received: {:?}", message.content);
        // deserialize and handle message
        let byte_slice = &message.content[..];
        let deserialized: Result<PlayerInputs, _> = serde_cbor::from_slice(byte_slice);
        println!("{:#?}", deserialized);
        match deserialized {
            Ok(PlayerInputs::CreateGame(game_creation_data)) => {
                //
            },
            _ => ()
        }
        // right now all we do is send it to everyone in the same room with this function:
        self.send_byte_message(&message.room, &message.content.clone(), message.sender_id);
    }
}

impl Handler<ListRooms> for GameServer {
    type Result = MessageResult<ListRooms>;
    fn handle(&mut self, _: ListRooms, _: &mut Context<Self>) -> Self::Result {
        let mut rooms = Vec::new();

        for key in self.rooms.keys() {
            rooms.push(key.to_owned())
        }

        MessageResult(rooms)
    }
}
