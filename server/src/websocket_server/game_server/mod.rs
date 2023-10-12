#![allow(dead_code, unused_imports)]
use actix::prelude::*;
use common::consts::{self, MAIN_CHAT_ROOM};
use common::errors::AppError;
use common::game::player_actions::PlayerInputs;
use common::game::RoguelikeRacerGame;
use common::packets::server_to_client::GameServerUpdatePackets;
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

#[derive(Debug)]
pub struct ConnectedUser {
    pub id: u32,
    pub actor_address: Recipient<AppMessage>,
    pub username: String,
    pub current_room_name: String,
    pub current_game_name: Option<String>,
}

impl ConnectedUser {
    pub fn new(id: u32, actor_address: Recipient<AppMessage>) -> Self {
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
    sessions: HashMap<u32, ConnectedUser>,
    rooms: HashMap<String, HashSet<u32>>,
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
pub fn get_user<'a>(
    sessions: &'a HashMap<u32, ConnectedUser>,
    actor_id: u32,
) -> Result<&'a ConnectedUser, AppError> {
    let user = sessions.get(&actor_id).ok_or(AppError {
        error_type: common::errors::AppErrorTypes::ServerError,
        message: consts::error_messages::USER_NOT_FOUND.to_string(),
    })?;
    Ok(user)
}
pub fn get_mut_user<'a>(
    sessions: &'a mut HashMap<u32, ConnectedUser>,
    actor_id: u32,
) -> Result<&'a mut ConnectedUser, AppError> {
    let user = sessions.get_mut(&actor_id).ok_or(AppError {
        error_type: common::errors::AppErrorTypes::ServerError,
        message: consts::error_messages::USER_NOT_FOUND.to_string(),
    })?;
    Ok(user)
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
        let result = match deserialized {
            Ok(PlayerInputs::CreateGame(game_creation_data)) => {
                self.create_game_handler(message.actor_id, game_creation_data)
            }
            Ok(PlayerInputs::JoinGame(game_name)) => {
                self.join_game_handler(message.actor_id, game_name)
            }
            Ok(PlayerInputs::LeaveGame) => self.leave_game_handler(message.actor_id),
            Ok(PlayerInputs::RequestGameList) => {
                self.game_list_update_request_handler(message.actor_id)
            }
            Ok(PlayerInputs::CreateAdventuringParty(party_name)) => {
                self.adventuring_party_creation_request_handler(message.actor_id, party_name)
            }
            Ok(PlayerInputs::LeaveAdventuringParty) => {
                self.leave_adventuring_party_handler(message.actor_id)
            }
            _ => {
                println! {"unhandled binary message\n {:#?}:",deserialized};
                Ok(())
            }
        };
        match result {
            Err(app_error) => {
                println!("{:#?}", app_error);
                match self.send_packet(
                    &GameServerUpdatePackets::Error(app_error.message),
                    message.actor_id,
                ) {
                    Err(app_error) => eprintln!("{:#?}", app_error),
                    _ => (),
                }
            }
            _ => (),
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
