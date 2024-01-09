#![allow(dead_code)]
use actix::prelude::*;
pub mod game_server;
pub mod websocket_actor;

pub enum MessageContent {
    Str(String),
    Bytes(Vec<u8>),
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct AppMessage(pub MessageContent);

#[derive(Message)]
#[rtype(result = "()")]
pub struct BinaryMessage {
    pub bytes: Vec<u8>,
}

#[derive(Message)]
#[rtype(u32)]
pub struct Connect {
    pub actor_id: u32,
    pub actor_address: Recipient<AppMessage>,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct Disconnect {
    pub actor_id: u32,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct ClientMessage {
    pub actor_id: u32,
    pub content: String,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct ClientBinaryMessage {
    pub actor_id: u32,
    pub content: Vec<u8>,
}

pub struct ListRooms;

impl actix::Message for ListRooms {
    type Result = Vec<String>;
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct Join {
    pub actor_id: u32,
    pub room_name: String,
}
