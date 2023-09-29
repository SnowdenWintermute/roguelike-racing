use super::GameServer;
use actix::prelude::*;
use actix::{Context, Handler};
use common::game::RoguelikeRacerGame;
use rand::{rngs::ThreadRng, Rng};
use std::collections::{HashMap, HashSet};
use std::sync::atomic::Ordering;

use crate::websocket_server::game_server::player_input_handler::leave_game_handler::leave_game_handler;
use crate::websocket_server::{Disconnect, MAIN_CHAT_ROOM};

impl Handler<Disconnect> for GameServer {
    type Result = ();
    fn handle(&mut self, message: Disconnect, _: &mut Context<Self>) {
        let Disconnect { actor_id } = message;
        println!("Actor with id {} disconnected", actor_id);
        let mut rooms: Vec<String> = Vec::new();
        self.visitor_count.fetch_sub(1, Ordering::SeqCst);

        leave_game_handler(self, actor_id);

        if self.sessions.remove(&actor_id).is_some() {
            for (room_name, sessions) in &mut self.rooms {
                if sessions.remove(&actor_id) {
                    rooms.push(room_name.to_owned());
                }
            }
        }

        for room in rooms {
            self.send_string_message(&room, "Someone disconnected");
        }
    }
}
