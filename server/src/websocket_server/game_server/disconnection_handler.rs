use super::GameServer;
use crate::websocket_server::Disconnect;
use actix::Context;
use actix::Handler;
use common::packets::server_to_client::GameServerUpdatePackets;
use common::packets::WebsocketChannelNamespace;

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
        let connected_user = connected_user.expect("is_none checked");
        let main_websocket_channel = connected_user.websocket_channels.main.clone();
        let party_websocket_channel = connected_user.websocket_channels.party.clone();
        let chat_websocket_channel = connected_user.websocket_channels.chat.clone();

        // remove them from all websocket channels
        let (main_channel_namespace, main_channel_name) = &main_websocket_channel;
        self.remove_user_from_websocket_channel(
            main_channel_name.as_str(),
            &main_channel_namespace,
            actor_id,
        );
        if let Some(party_channel_name) = &party_websocket_channel {
            self.remove_user_from_websocket_channel(
                party_channel_name.as_str(),
                &WebsocketChannelNamespace::Party,
                actor_id,
            );
        }
        if let Some(chat_channel_name) = &chat_websocket_channel {
            self.remove_user_from_websocket_channel(
                chat_channel_name.as_str(),
                &WebsocketChannelNamespace::Chat,
                actor_id,
            );
        }

        // remove them from their game if any

        if let Ok(player_and_game) = self.remove_player_from_game(actor_id) {
            let result = self.emit_packet(
                player_and_game.game_name.as_str(),
                &main_channel_namespace,
                &GameServerUpdatePackets::UserLeftGame(player_and_game.username.clone()),
                Some(actor_id),
            );
            if result.is_err() {
                eprintln!("{:#?}", result)
            }
        };

        // remove them from list of connected users
        self.sessions.remove(&actor_id);
    }
}
