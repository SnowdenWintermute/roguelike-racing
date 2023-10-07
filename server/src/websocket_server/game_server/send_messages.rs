use common::packets::server_to_client::GameServerUpdatePackets;

use super::GameServer;
use crate::websocket_server::{AppMessage, MessageContent};

impl GameServer {
    pub fn send_string_message(&self, room: &str, message: &str) {
        if let Some(sessions) = self.rooms.get(room) {
            for id in sessions {
                // optionally skip sending to same socket
                if let Some(connected_user) = self.sessions.get(id) {
                    connected_user
                        .actor_address
                        .do_send(AppMessage(MessageContent::Str(message.to_owned())));
                }
            }
        }
    }

    pub fn send_byte_message(&self, room: &str, message: &Vec<u8>) {
        if let Some(sessions) = self.rooms.get(room) {
            for id in sessions {
                if let Some(connected_user) = self.sessions.get(id) {
                    connected_user
                        .actor_address
                        .do_send(AppMessage(MessageContent::Bytes(message.to_vec())));
                }
            }
        }
    }

    pub fn send_packet(&self, packet: &GameServerUpdatePackets, actor_id: u32) {
        if let Some(connected_user) = self.sessions.get(&actor_id) {
            let serialized = serde_cbor::to_vec(&packet);

            match serialized {
                Ok(bytes) => connected_user
                    .actor_address
                    .do_send(AppMessage(MessageContent::Bytes(bytes))),
                Err(_e) => println!("error serializing full update"),
            }
        } else {
            println!("tried to send a packet to a client but couldn't find any connected user with the provide actor_id")
        }
    }

    pub fn emit_packet(&self, room: &str, packet: &GameServerUpdatePackets, skip_id: Option<u32>) {
        if let Some(sessions) = self.rooms.get(room) {
            for actor_id in sessions {
                if let Some(id_to_skip) = skip_id {
                    if &id_to_skip == actor_id {
                        continue;
                    }
                }
                self.send_packet(packet, *actor_id)
            }
        }
    }

    pub fn send_lobby_and_game_full_updates(&self, actor_id: u32) {
        let full_update = GameServer::create_client_update_packet(&self, actor_id)
            .expect("failed to create full client update");
        if let Some(connected_user) = self.sessions.get(&actor_id) {
            let serialized = serde_cbor::to_vec(&full_update);
            match serialized {
                Ok(bytes) => connected_user
                    .actor_address
                    .do_send(AppMessage(MessageContent::Bytes(bytes))),
                Err(_e) => println!("error serializing full update"),
            }
        }
    }
}
