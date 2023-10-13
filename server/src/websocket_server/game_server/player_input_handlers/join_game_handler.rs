use crate::websocket_server::game_server::{
    getters::{get_mut_game, get_mut_user},
    GameServer,
};
use common::{
    app_consts::error_messages, errors::AppError, game::RoguelikeRacerPlayer,
    packets::server_to_client::GameServerUpdatePackets,
};

impl GameServer {
    // @TODO
    // reject if game is full (and define what that means)
    pub fn join_game_handler(&mut self, actor_id: u32, game_name: String) -> Result<(), AppError> {
        let game = get_mut_game(&mut self.games, &game_name)?;
        let connected_user = get_mut_user(&mut self.sessions, actor_id)?;
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
        self.join_room_handler(&game_name, actor_id)?;

        let game_update = self.create_game_full_update(actor_id)?;
        self.send_packet(
            &GameServerUpdatePackets::GameFullUpdate(game_update),
            actor_id,
        )?;
        self.emit_packet(
            game_name.as_str(),
            &GameServerUpdatePackets::UserJoinedGame(username),
            Some(actor_id),
        )
    }
}
