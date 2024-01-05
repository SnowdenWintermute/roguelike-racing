use super::getters::get_mut_user;
use super::GameServer;
use common::errors::AppError;
use common::packets::server_to_client::GameServerUpdatePackets;
use common::packets::server_to_client::WebsocketChannelAndUserPacket;
use common::packets::WebsocketChannelNamespace;

impl GameServer {
    pub fn remove_user_from_websocket_channel(
        &mut self,
        channel_name: &str,
        namespace: &WebsocketChannelNamespace,
        actor_id: u32,
    ) -> Result<(), AppError> {
        let connected_user = get_mut_user(&mut self.sessions, actor_id)?;
        let username = connected_user.username.clone();

        match namespace {
            WebsocketChannelNamespace::Party => {
                connected_user.websocket_channels.party = None;
            }
            WebsocketChannelNamespace::Chat => {
                connected_user.websocket_channels.chat = None;
            }
            _ => (),
        }

        self.emit_packet(
            &channel_name,
            &namespace,
            &GameServerUpdatePackets::UserLeftWebsocketChannel(WebsocketChannelAndUserPacket {
                username,
                channel_name: channel_name.to_string(),
                channel_namespace: namespace.clone(),
            }),
            Some(actor_id),
        );

        self.websocket_channels
            .entry(namespace.clone())
            .or_default()
            .entry(channel_name.to_string())
            .or_default()
            .remove(&actor_id);

        if self
            .websocket_channels
            .entry(namespace.clone())
            .or_default()
            .entry(channel_name.to_string())
            .or_default()
            .len()
            == 0
        {
            self.websocket_channels
                .entry(namespace.clone())
                .or_default()
                .remove(&channel_name.to_string());
        }

        Ok(())
    }
}
