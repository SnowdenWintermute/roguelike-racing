use actix::prelude::*;
use rand::{self, rngs::ThreadRng, Rng};
use std::collections::{HashMap, HashSet};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
pub mod session;

pub const MAIN_CHAT_ROOM: &str = "main";

pub enum MessageContent {
    Str(String),
    Bytes(Vec<u8>),
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct Message(pub MessageContent);

#[derive(Message)]
#[rtype(result = "()")]
pub struct BinaryMessage {
    pub bytes: Vec<u8>,
}

#[derive(Message)]
#[rtype(usize)]
pub struct Connect {
    pub session_address: Recipient<Message>,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct Disconnect {
    pub sender_id: usize,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct ClientMessage {
    pub sender_id: usize,
    pub content: String,
    pub room: String,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct ClientBinaryMessage {
    pub sender_id: usize,
    pub content: Vec<u8>,
    pub room: String,
}

pub struct ListRooms;

impl actix::Message for ListRooms {
    type Result = Vec<String>;
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct Join {
    pub sender_id: usize,
    pub room_name: String,
}

#[derive(Debug)]
pub struct ChatServer {
    sessions: HashMap<usize, Recipient<Message>>,
    rooms: HashMap<String, HashSet<usize>>,
    rng: ThreadRng,
    visitor_count: Arc<AtomicUsize>,
}

impl ChatServer {
    pub fn new(visitor_count: Arc<AtomicUsize>) -> ChatServer {
        let mut rooms = HashMap::new();
        rooms.insert(MAIN_CHAT_ROOM.to_owned(), HashSet::new());

        ChatServer {
            sessions: HashMap::new(),
            rooms,
            rng: rand::thread_rng(),
            visitor_count,
        }
    }
}

impl ChatServer {
    fn send_message(&self, room: &str, message: &str, skip_id: usize) {
        if let Some(sessions) = self.rooms.get(room) {
            for id in sessions {
                if *id == skip_id {
                    continue;
                }

                if let Some(session_address) = self.sessions.get(id) {
                    session_address.do_send(Message(MessageContent::Str(message.to_owned())));
                }
            }
        }
    }

    fn send_byte_message(&self, room: &str, message: &Vec<u8>, skip_id: usize) {
        if let Some(sessions) = self.rooms.get(room) {
            for id in sessions {
                if *id == skip_id {
                    continue;
                }
                if let Some(addr) = self.sessions.get(id) {
                    addr.do_send(Message(MessageContent::Bytes(message.to_vec())));
                }
            }
        }
    }
}

impl Actor for ChatServer {
    /// We are going to use simple Context, we just need ability to communicate
    /// with other actors.
    type Context = Context<Self>;
}

impl Handler<Connect> for ChatServer {
    type Result = usize;
    fn handle(&mut self, message: Connect, _: &mut Context<Self>) -> Self::Result {
        let id = self.rng.gen::<usize>();
        self.sessions.insert(id, message.session_address);
        println!("Someone joined, assigned the id {}", id);

        self.send_message(MAIN_CHAT_ROOM, "Someone joined", 0);

        self.rooms
            .entry(MAIN_CHAT_ROOM.to_owned())
            .or_default()
            .insert(id);

        let count = self.visitor_count.fetch_add(1, Ordering::SeqCst);
        self.send_message(MAIN_CHAT_ROOM, &format!("Total visitors {count}"), 0);

        id
    }
}

impl Handler<Disconnect> for ChatServer {
    type Result = ();
    fn handle(&mut self, message: Disconnect, _: &mut Context<Self>) {
        println!("Someone disconnected");
        let mut rooms: Vec<String> = Vec::new();

        if self.sessions.remove(&message.sender_id).is_some() {
            for (room_name, sessions) in &mut self.rooms {
                if sessions.remove(&message.sender_id) {
                    rooms.push(room_name.to_owned());
                }
            }
        }

        for room in rooms {
            self.send_message(&room, "Someone disconnected", 0);
        }
    }
}

impl Handler<ClientMessage> for ChatServer {
    type Result = ();
    fn handle(&mut self, message: ClientMessage, _: &mut Context<Self>) {
        println!("message received: {}", message.content);
        self.send_message(&message.room, message.content.as_str(), message.sender_id);
    }
}

impl Handler<ClientBinaryMessage> for ChatServer {
    type Result = ();
    fn handle(&mut self, message: ClientBinaryMessage, _: &mut Context<Self>) {
        println!("message received: {:?}", message.content);
        self.send_byte_message(&message.room, &message.content.clone(), message.sender_id);
    }
}

impl Handler<ListRooms> for ChatServer {
    type Result = MessageResult<ListRooms>;
    fn handle(&mut self, _: ListRooms, _: &mut Context<Self>) -> Self::Result {
        let mut rooms = Vec::new();

        for key in self.rooms.keys() {
            rooms.push(key.to_owned())
        }

        MessageResult(rooms)
    }
}

impl Handler<Join> for ChatServer {
    type Result = ();
    fn handle(&mut self, message: Join, _: &mut Context<Self>) {
        let Join {
            sender_id,
            room_name,
        } = message;
        let mut rooms = Vec::new();

        for (n, sessions) in &mut self.rooms {
            if sessions.remove(&sender_id) {
                rooms.push(n.to_owned());
            }
        }

        for room in rooms {
            self.send_message(&room, "Someone disconnected", 0);
        }

        self.rooms
            .entry(room_name.clone())
            .or_default()
            .insert(sender_id);

        self.send_message(&room_name, "Someone connected", sender_id);
    }
}
