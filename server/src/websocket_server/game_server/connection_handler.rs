#![allow(dead_code)]
use super::GameServer;
use crate::websocket_server::{game_server::ConnectedUser, Connect};
use actix::{Context, Handler};
use common::{app_consts::MAIN_CHAT_ROOM, packets::server_to_client::GameServerUpdatePackets};

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

        let full_update = GameServer::create_client_update_packet(self, actor_id);
        match full_update {
            Ok(update) => {
                let result =
                    self.send_packet(&GameServerUpdatePackets::FullUpdate(update), actor_id);
                if result.is_err() {
                    eprintln!("{:#?}", result)
                }
            }
            Err(e) => {
                eprintln!("{:#?}", e)
            }
        }

        actor_id
    }
}
