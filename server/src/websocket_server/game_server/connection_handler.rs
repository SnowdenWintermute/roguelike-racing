#![allow(dead_code)]
use super::GameServer;
use crate::websocket_server::{game_server::ConnectedUser, Connect};
use actix::{Context, Handler};
use common::consts::MAIN_CHAT_ROOM;
use rand::{self, Rng};
use std::sync::atomic::Ordering;

impl Handler<Connect> for GameServer {
    type Result = u32;
    fn handle(&mut self, message: Connect, _: &mut Context<Self>) -> Self::Result {
        let Connect {
            actor_id,
            actor_address,
        } = message;

        let new_user_connection = ConnectedUser::new(actor_id, actor_address);
        self.sessions.insert(actor_id, new_user_connection);
        println!("actor id {} connected", actor_id);

        let result = self.join_room_handler(MAIN_CHAT_ROOM, actor_id);
        if result.is_err() {
            eprintln!("{:#?}", result)
        }

        let result = self.send_lobby_and_game_full_updates(actor_id);
        if result.is_err() {
            eprintln!("{:#?}", result)
        }

        actor_id
    }
}
