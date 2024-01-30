#![allow(unused)]
use super::getters::get_user;
use super::GameServer;
use crate::websocket_server::AppMessage;
use crate::websocket_server::MessageContent;
use common::app_consts::error_messages;
use common::errors::AppError;
use common::packets::server_to_client::GameServerUpdatePackets;
use common::packets::WebsocketChannelNamespace;

impl GameServer {
    pub fn send_packet(
        &self,
        packet: &GameServerUpdatePackets,
        actor_id: u32,
    ) -> Result<(), AppError> {
        let connected_user = get_user(&self.sessions, actor_id)?;
        let serialized = serde_cbor::to_vec(&packet)?;

        connected_user
            .websocket_actor
            .do_send(AppMessage(MessageContent::Bytes(serialized)));
        Ok(())
    }

    pub fn emit_packet(
        &self,
        websocket_channel: &str,
        channel_namespace: &WebsocketChannelNamespace,
        packet: &GameServerUpdatePackets,
        skip_id: Option<u32>,
    ) -> Result<(), AppError> {
        println!(
            "emitting packet on channel :{websocket_channel} in namespace: {:?} packet: {:#?}",
            channel_namespace, packet
        );
        let channels_in_namespace =
            self.websocket_channels
                .get(channel_namespace)
                .ok_or_else(|| AppError {
                    error_type: common::errors::AppErrorTypes::ServerError,
                    message: error_messages::WEBSOCKET_NAMESPACE_NOT_FOUND.to_string(),
                })?;
        let sessions = channels_in_namespace
            .get(websocket_channel)
            .ok_or_else(|| AppError {
                error_type: common::errors::AppErrorTypes::ServerError,
                message: error_messages::WEBSOCKET_CHANNEL_NOT_FOUND.to_string(),
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

    pub fn send_string_message(&self, room: &str, message: &str) -> Result<(), AppError> {
        // let sessions = self.rooms.get(room).ok_or_else(|| AppError {
        //     error_type: common::errors::AppErrorTypes::ServerError,
        //     message: error_messages::ROOM_NOT_FOUND.to_string(),
        // })?;
        // for actor_id in sessions {
        //     let connected_user = get_user(&self.sessions, *actor_id)?;
        //     connected_user
        //         .websocket_actor
        //         .do_send(AppMessage(MessageContent::Str(message.to_owned())));
        // }
        Ok(())
    }

    // pub fn send_byte_message(&self, room: &str, message: &Vec<u8>) -> Result<(), AppError> {
    //     let sessions = self.rooms.get(room).ok_or_else(||AppError {
    //         error_type: common::errors::AppErrorTypes::ServerError,
    //         message: common::consts::error_messages::ROOM_NOT_FOUND.to_string(),
    //     })?;
    //     for actor_id in sessions {
    //         let connected_user = get_user(&self.sessions, *actor_id)?;
    //         connected_user
    //             .websocket_actor
    //             .do_send(AppMessage(MessageContent::Bytes(message.to_vec())));
    //     }
    //     Ok(())
    // }
}
