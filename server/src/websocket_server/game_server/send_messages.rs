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
}
