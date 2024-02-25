use super::GameServer;
use crate::websocket_server::game_server::ConnectedUser;
use crate::websocket_server::Connect;
use actix::Context;
use actix::Handler;
use common::app_consts::LOBBY_CHANNEL;
use common::packets::server_to_client::GameServerUpdatePackets;
use common::packets::WebsocketChannelNamespace;
use common::utils::server_log;

impl Handler<Connect> for GameServer {
    type Result = u32;
    fn handle(&mut self, message: Connect, _: &mut Context<Self>) -> Self::Result {
        let Connect {
            actor_id,
            actor_address,
        } = message;

        let new_user_connection = ConnectedUser::new(actor_id, actor_address);
        let username = new_user_connection.username.clone();
        self.sessions.insert(actor_id, new_user_connection);
        server_log(&format!("actor id {} connected", actor_id));

        let result = self.join_user_to_websocket_channel(
            LOBBY_CHANNEL,
            WebsocketChannelNamespace::Lobby,
            actor_id,
        );
        if result.is_err() {
            server_log(&format!("{:#?}", result))
        }

        let full_update = GameServer::create_client_update_packet(self, actor_id);
        match full_update {
            Ok(update) => {
                let result =
                    self.send_packet(&GameServerUpdatePackets::FullUpdate(update), actor_id);
                if result.is_err() {
                    server_log(&format!("{:#?}", result))
                }
            }
            Err(e) => server_log(&format!("{:#?}", e)),
        }

        let _ = self.send_packet(&GameServerUpdatePackets::ClientUserName(username), actor_id);

        actor_id
    }
}
