use actix::prelude::{Message, Recipient};
use uuid::Uuid;

#[derive(Message)]
#[rtype(result = "()")]
pub struct WsMessage(pub String);

#[derive(Message)]
#[rtype(result = "()")]
pub struct Connect {
    pub address: Recipient<WsMessage>,
    pub lobby_id: Uuid,
    pub sender_id: Uuid,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct Disconnect {
    pub room_id: Uuid,
    pub sender_id: Uuid,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct ClientActorMessage {
    pub sender_id: Uuid,
    pub content: String,
    pub room_id: Uuid,
}
