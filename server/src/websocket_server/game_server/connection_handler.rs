#![allow(dead_code)]
use actix::{Context, Handler};
use rand::{self, Rng};
use std::sync::atomic::Ordering;

use crate::websocket_server::{Connect, MAIN_CHAT_ROOM};

use super::GameServer;

impl Handler<Connect> for GameServer {
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
