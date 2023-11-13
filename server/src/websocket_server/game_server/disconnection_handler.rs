use super::GameServer;
use crate::websocket_server::Disconnect;
use actix::{Context, Handler};
use common::packets::server_to_client::GameServerUpdatePackets;

impl Handler<Disconnect> for GameServer {
    type Result = ();
    fn handle(&mut self, message: Disconnect, _: &mut Context<Self>) {
        let Disconnect { actor_id } = message;
        println!("Actor with id {} disconnected", actor_id);

        let connected_user = self.sessions.get(&actor_id);
        if connected_user.is_none() {
            println!("a user disconnected but they weren't in the server's list of users");
            return;
        }

        let room_name_leaving = connected_user.unwrap().current_room_name.clone();
        let username = connected_user.unwrap().username.clone();

        if let Ok(player_and_game) = self.remove_player_from_game(actor_id) {
            let result = self.emit_packet(
                player_and_game.game_name.as_str(),
                &GameServerUpdatePackets::UserLeftGame(player_and_game.username.clone()),
                Some(actor_id),
            );
            if result.is_err() {
                eprintln!("{:#?}", result)
            }
        };

        let room_leaving = self.rooms.get_mut(&room_name_leaving);
        match room_leaving {
            Some(room) => {
                room.remove(&actor_id);
                // UPDATE THEIR PREVIOUS ROOM MEMBERS
                let result = self.emit_packet(
                    &room_name_leaving,
                    &GameServerUpdatePackets::UserLeftRoom(username),
                    None,
                );
                if result.is_err() {
                    eprintln!("{:#?}", result)
                }
            }
            None => println!("tried to remove a user from a room but no room was found"),
        }

        self.sessions.remove(&actor_id);
    }
}
