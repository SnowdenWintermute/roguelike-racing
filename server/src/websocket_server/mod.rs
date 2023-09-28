use actix::prelude::*;
pub mod game_server;
pub mod session;

pub const MAIN_CHAT_ROOM: &str = "main";

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
#[rtype(usize)]
pub struct Connect {
    pub actor_id: usize,
    pub actor_address: Recipient<AppMessage>,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct Disconnect {
    pub actor_id: usize,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct ClientMessage {
    pub actor_id: usize,
    pub content: String,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct ClientBinaryMessage {
    pub actor_id: usize,
    pub content: Vec<u8>,
}

pub struct ListRooms;

impl actix::Message for ListRooms {
    type Result = Vec<String>;
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct Join {
    pub actor_id: usize,
    pub room_name: String,
}
