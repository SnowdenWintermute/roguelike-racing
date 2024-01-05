use crate::websocket_server::game_server::getters::get_user;
use crate::websocket_server::game_server::GameServer;
use common::app_consts::error_messages;
use common::errors::AppError;
use common::packets::WebsocketChannelNamespace;
use std::collections::HashSet;

impl GameServer {
    pub fn get_users_in_websocket_channel(
        &self,
        namespace: &WebsocketChannelNamespace,
        channel_name: &String,
    ) -> Result<HashSet<String>, AppError> {
        let channels_in_namespace =
            self.websocket_channels
                .get(&namespace)
                .ok_or_else(|| AppError {
                    error_type: common::errors::AppErrorTypes::ServerError,
                    message: error_messages::WEBSOCKET_NAMESPACE_NOT_FOUND.to_string(),
                })?;

        let actor_ids_in_channel =
            channels_in_namespace
                .get(channel_name)
                .ok_or_else(|| AppError {
                    error_type: common::errors::AppErrorTypes::ServerError,
                    message: error_messages::WEBSOCKET_CHANNEL_NOT_FOUND.to_string(),
                })?;

        let mut usernames_in_channel = HashSet::new();
        for actor_id in actor_ids_in_channel.iter() {
            let user = get_user(&self.sessions, *actor_id)?;
            usernames_in_channel.insert(user.username.clone());
        }

        Ok(usernames_in_channel)
    }
}
