use super::GameServer;
use crate::websocket_server::Disconnect;
use actix::prelude::*;
use actix::{Context, Handler};
use common::game::RoguelikeRacerGame;
use common::packets::server_to_client::GameServerUpdatePackets;
use rand::{rngs::ThreadRng, Rng};
use std::collections::{HashMap, HashSet};
use std::sync::atomic::Ordering;

impl Handler<Disconnect> for GameServer {
    type Result = ();
    fn handle(&mut self, message: Disconnect, _: &mut Context<Self>) {
        let Disconnect { actor_id } = message;
        println!("Actor with id {} disconnected", actor_id);

        let connected_user = self.sessions.get(&actor_id);
        if connected_user.is_none() {
            println!("a user disconnected but they weren't in the server's list of users")
        }

        let room_name_leaving = connected_user.unwrap().current_room_name.clone();
        let username = connected_user.unwrap().username.clone();

        if let Ok(player_and_game) = self.remove_player_from_game(actor_id) {
            self.emit_packet(
                player_and_game.game_name.as_str(),
                &GameServerUpdatePackets::UserLeftGame(player_and_game.username.clone()),
                Some(actor_id),
            );
        };

        let room_leaving = self.rooms.get_mut(&room_name_leaving);
        match room_leaving {
            Some(room) => {
                room.remove(&actor_id);
                // UPDATE THEIR PREVIOUS ROOM MEMBERS
                self.emit_packet(
                    &room_name_leaving,
                    &GameServerUpdatePackets::UserLeftRoom(username),
                    None,
                );
            }
            None => println!("tried to remove a user from a room but no room was found"),
        }

        self.sessions.remove(&actor_id);
    }
}
