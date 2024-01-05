use crate::websocket_server::game_server::getters::get_mut_game;
use crate::websocket_server::game_server::getters::get_mut_user;
use crate::websocket_server::game_server::GameServer;
use common::app_consts::error_messages;
use common::errors::AppError;
use common::game::RoguelikeRacerPlayer;
use common::packets::server_to_client::GameServerUpdatePackets;
use common::packets::WebsocketChannelNamespace;

impl GameServer {
    // @TODO
    // reject if game is full (and define what that means)
    pub fn join_game_handler(&mut self, actor_id: u32, game_name: String) -> Result<(), AppError> {
        let game = get_mut_game(&mut self.games, &game_name)?;
        let connected_user = get_mut_user(&mut self.sessions, actor_id)?;
        let (current_main_channel_namespace, current_main_channel_name) =
            connected_user.websocket_channels.main.clone();
        if connected_user.current_game_name.is_some() {
            return Err(AppError {
                error_type: common::errors::AppErrorTypes::InvalidInput,
                message: error_messages::ALREADY_IN_GAME.to_string(),
            });
        }

        if game.time_started.is_some() {
            return Err(AppError {
                error_type: common::errors::AppErrorTypes::InvalidInput,
                message: error_messages::GAME_HAS_STARTED.to_string(),
            });
        }
        let username = connected_user.username.clone();

        let new_player =
            RoguelikeRacerPlayer::new(Some(actor_id), connected_user.username.to_string());
        game.players
            .insert(connected_user.username.to_string(), new_player);
        connected_user.current_game_name = Some(game_name.to_string());

        self.remove_user_from_websocket_channel(
            &current_main_channel_name,
            &current_main_channel_namespace,
            actor_id,
        )?;
        self.join_user_to_websocket_channel(&game_name, WebsocketChannelNamespace::Game, actor_id)?;

        let game_update = self.create_game_full_update(actor_id)?;

        self.send_packet(
            &GameServerUpdatePackets::GameFullUpdate(game_update),
            actor_id,
        )?;
        self.emit_packet(
            game_name.as_str(),
            &WebsocketChannelNamespace::Game,
            &GameServerUpdatePackets::UserJoinedGame(username),
            Some(actor_id),
        )
    }
}
