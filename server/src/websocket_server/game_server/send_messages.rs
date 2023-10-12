use common::{errors::AppError, packets::server_to_client::GameServerUpdatePackets};

use super::{get_user, GameServer};
use crate::websocket_server::{AppMessage, MessageContent};

impl GameServer {
    pub fn send_string_message(&self, room: &str, message: &str) -> Result<(), AppError> {
        if let Some(sessions) = self.rooms.get(room) {
            for actor_id in sessions {
                let connected_user = get_user(&self.sessions, *actor_id)?;
                connected_user
                    .actor_address
                    .do_send(AppMessage(MessageContent::Str(message.to_owned())));
            }
        }
        Ok(())
    }

    pub fn send_byte_message(&self, room: &str, message: &Vec<u8>) -> Result<(), AppError> {
        if let Some(sessions) = self.rooms.get(room) {
            for actor_id in sessions {
                let connected_user = get_user(&self.sessions, *actor_id)?;
                connected_user
                    .actor_address
                    .do_send(AppMessage(MessageContent::Bytes(message.to_vec())));
            }
        }
        Ok(())
    }

    pub fn send_packet(
        &self,
        packet: &GameServerUpdatePackets,
        actor_id: u32,
    ) -> Result<(), AppError> {
        let connected_user = get_user(&self.sessions, actor_id)?;
        let serialized = serde_cbor::to_vec(&packet)?;

        connected_user
            .actor_address
            .do_send(AppMessage(MessageContent::Bytes(serialized)));
        Ok(())
    }

    pub fn emit_packet(
        &self,
        room: &str,
        packet: &GameServerUpdatePackets,
        skip_id: Option<u32>,
    ) -> Result<(), AppError> {
        let sessions = self.rooms.get(room).ok_or(AppError {
            error_type: common::errors::AppErrorTypes::ServerError,
            message: "Tried to emit a message in a room that doesn't exist".to_string(),
        })?;
        for actor_id in sessions {
            if let Some(id_to_skip) = skip_id {
                if &id_to_skip == actor_id {
                    continue;
                }
            }
            self.send_packet(packet, *actor_id)?;
        }
        Ok(())
    }

    pub fn send_lobby_and_game_full_updates(&mut self, actor_id: u32) -> Result<(), AppError> {
        let full_update = GameServer::create_client_update_packet(self, actor_id)?;
        let connected_user = get_user(&self.sessions, actor_id)?;
        let serialized = serde_cbor::to_vec(&full_update)?;
        connected_user
            .actor_address
            .do_send(AppMessage(MessageContent::Bytes(serialized)));
        Ok(())
    }
}
