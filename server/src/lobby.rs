use crate::messages::{ClientActorMessage, Connect, Disconnect, WsMessage};
use actix::prelude::{Actor, Context, Handler, Recipient};
use std::collections::{HashMap, HashSet};
use uuid::Uuid;

type Socket = Recipient<WsMessage>;

pub struct Lobby {
    sessions: HashMap<Uuid, Socket>,
    rooms: HashMap<Uuid, HashSet<Uuid>>,
}

impl Default for Lobby {
    fn default() -> Lobby {
        Lobby {
            sessions: HashMap::new(),
            rooms: HashMap::new(),
        }
    }
}

impl Lobby {
    fn send_message(&self, message: &str, id_to: &Uuid) {
        if let Some(socket_recipient) = self.sessions.get(id_to) {
            let _ = socket_recipient.do_send(WsMessage(message.to_owned()));
        } else {
            println!("attempted to send message but couldn't find recipient's id")
        }
    }
}

impl Actor for Lobby {
    type Context = Context<Self>;
}

impl Handler<Disconnect> for Lobby {
    type Result = ();

    fn handle(&mut self, message: Disconnect, _: &mut Context<Self>) {
        if self.sessions.remove(&message.sender_id).is_some() {
            self.rooms
                .get(&message.room_id)
                .unwrap()
                .iter()
                .filter(|conn_id| *conn_id.to_owned() != message.sender_id)
                .for_each(|user_id| {
                    self.send_message(&format!("{} disconnected", &message.sender_id), user_id)
                });
            if let Some(lobby) = self.rooms.get_mut(&message.room_id) {
                if lobby.len() > 1 {
                    lobby.remove(&message.sender_id);
                } else {
                    self.rooms.remove(&message.room_id)
                }
            }
        }
    }
}

impl Handler<Connect> for Lobby {
    type Result = ();

    fn handle(&mut self, message: Connect, context: &mut Self::Context) -> Self::Result {
        self.rooms
            .entry(message.lobby_id)
            .or_insert_with(HashSet::new)
            .insert(message.sender_id);

        self.rooms
            .get(&message.lobby_id)
            .unwrap()
            .iter()
            .filter(|conn_id| *conn_id.to_owned() != message.sender_id)
            .for_each(|conn_id| {
                self.send_message(&format!("{} just joined!", message.sender_id), conn_id)
            });

        self.sessions.insert(message.self_id, message.address);

        self.send_message(
            &format!("your id is {}", message.sender_id),
            &message.sender_id,
        )
    }
}

impl Handler<ClientActorMessage> for Lobby {
    type Result = ();

    fn handle(&mut self, message: ClientActorMessage, _: &mut Context<Self>) {
        self.rooms
            .get(&message.room_id)
            .unwrap()
            .iter()
            .for_each(|client| self.send_message(&message.content, client))
    }
}
