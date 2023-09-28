#![allow(dead_code)]
use super::GameServer;
use crate::websocket_server::{game_server::ConnectedUser, Connect, MAIN_CHAT_ROOM};
use actix::{Context, Handler};
use rand::{self, Rng};
use std::sync::atomic::Ordering;

impl Handler<Connect> for GameServer {
    type Result = usize;
    fn handle(&mut self, message: Connect, _: &mut Context<Self>) -> Self::Result {
        let Connect {
            actor_id,
            actor_address,
        } = message;

        let new_user_connection_data = ConnectedUser::new(actor_id, actor_address);
        self.sessions.insert(actor_id, new_user_connection_data);
        println!("actor id {} connected", actor_id);

        self.send_string_message(MAIN_CHAT_ROOM, "Someone joined");

        self.rooms
            .entry(MAIN_CHAT_ROOM.to_owned())
            .or_default()
            .insert(actor_id);

        let count = self.visitor_count.fetch_add(1, Ordering::SeqCst);
        self.send_string_message(MAIN_CHAT_ROOM, &format!("Total visitors {count}"));

        actor_id
    }
}
