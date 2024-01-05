use super::getters::get_mut_user;
use super::GameServer;
use super::WebsocketChannelNamespace;
use common::app_consts::error_messages;
use common::errors::AppError;
use common::packets::server_to_client::GameServerUpdatePackets;
use common::packets::server_to_client::WebsocketChannelAndUserPacket;
use common::packets::server_to_client::WebsocketChannelFullState;

impl GameServer {
    pub fn join_user_to_websocket_channel(
        &mut self,
        channel_name: &str,
        namespace: WebsocketChannelNamespace,
        actor_id: u32,
    ) -> Result<(), AppError> {
        let connected_user = get_mut_user(&mut self.sessions, actor_id)?;
        let username = connected_user.username.clone();

        self.websocket_channels
            .entry(namespace.clone())
            .or_default()
            .entry(channel_name.to_string())
            .or_default()
            .insert(actor_id);

        let namespace_of_joined_channel =
            self.websocket_channels
                .get(&namespace)
                .ok_or_else(|| AppError {
                    error_type: common::errors::AppErrorTypes::ServerError,
                    message: error_messages::WEBSOCKET_NAMESPACE_NOT_FOUND.to_string(),
                })?;
        let channel_joined = namespace_of_joined_channel
            .get(channel_name)
            .ok_or_else(|| AppError {
                error_type: common::errors::AppErrorTypes::ServerError,
                message: error_messages::WEBSOCKET_CHANNEL_NOT_FOUND.to_string(),
            })?;

        match namespace {
            WebsocketChannelNamespace::Lobby | WebsocketChannelNamespace::Game => {
                connected_user.websocket_channels.main =
                    (namespace.clone(), channel_name.to_string());
            }
            WebsocketChannelNamespace::Party => {
                connected_user.websocket_channels.party = Some(channel_name.to_string());
            }
            WebsocketChannelNamespace::Chat => {
                connected_user.websocket_channels.chat = Some(channel_name.to_string());
            }
        }

        let usernames_in_joined_channel = channel_joined
            .into_iter()
            .filter_map(|id| {
                if let Some(connected_user) = self.sessions.get(id) {
                    Some(connected_user.username.clone())
                } else {
                    None
                }
            })
            .collect();

        self.send_packet(
            &GameServerUpdatePackets::WebsocketChannelFullUpdate(WebsocketChannelFullState {
                name: channel_name.to_string(),
                namespace: namespace.clone(),
                usernames_in_channel: usernames_in_joined_channel,
            }),
            actor_id,
        )?;

        self.emit_packet(
            &channel_name,
            &namespace,
            &GameServerUpdatePackets::UserJoinedWebsocketChannel(WebsocketChannelAndUserPacket {
                username,
                channel_name: channel_name.to_string(),
                channel_namespace: namespace.clone(),
            }),
            Some(actor_id),
        )
    }
}
