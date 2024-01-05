use crate::websocket_server::game_server::getters::get_mut_game;
use crate::websocket_server::game_server::getters::get_mut_user;
use crate::websocket_server::game_server::GameServer;
use common::app_consts::error_messages;
use common::errors::AppError;
use common::game::getters::get_mut_player;
use common::packets::server_to_client::GameServerUpdatePackets;
use common::packets::server_to_client::PlayerAdventuringPartyChange;
use common::packets::WebsocketChannelNamespace;

impl GameServer {
    pub fn leave_party_handler(&mut self, actor_id: u32) -> Result<(), AppError> {
        let connected_user = get_mut_user(&mut self.sessions, actor_id)?;
        let username = connected_user.username.clone();
        let current_game_name =
            connected_user
                .current_game_name
                .clone()
                .ok_or_else(|| AppError {
                    error_type: common::errors::AppErrorTypes::ServerError,
                    message: error_messages::MISSING_GAME_REFERENCE.to_string(),
                })?;

        let game = get_mut_game(&mut self.games, &current_game_name)?;
        let player = get_mut_player(game, &username)?;
        let username = player.username.clone();
        let party_id_leaving_option =
            game.remove_player_from_adventuring_party(username.clone())?;
        let party_websocket_channel_name_option =
            if let Some(party_id_leaving) = party_id_leaving_option {
                Some(game.get_party_channel_name(party_id_leaving))
            } else {
                None
            };

        if let Some(party_websocket_channel_name) = party_websocket_channel_name_option {
            self.remove_user_from_websocket_channel(
                &party_websocket_channel_name,
                &WebsocketChannelNamespace::Party,
                actor_id,
            )?;

            self.send_packet(
                &GameServerUpdatePackets::ClientAdventuringPartyId(None),
                actor_id,
            )?;

            self.emit_packet(
                &current_game_name,
                &WebsocketChannelNamespace::Game,
                &GameServerUpdatePackets::PlayerChangedAdventuringParty(
                    PlayerAdventuringPartyChange {
                        username: username.clone(),
                        party_id: None,
                    },
                ),
                None,
            )?;
        }

        Ok(())
    }
}
