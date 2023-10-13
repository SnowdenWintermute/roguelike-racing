use crate::websocket_server::game_server::getters::{get_mut_game, get_mut_user};
use crate::websocket_server::game_server::GameServer;
use common::app_consts::error_messages;
use common::errors::AppError;
use common::packets::server_to_client::{GameServerUpdatePackets, PlayerAdventuringPartyChange};

impl GameServer {
    pub fn leave_adventuring_party_handler(&mut self, actor_id: u32) -> Result<(), AppError> {
        let connected_user = get_mut_user(&mut self.sessions, actor_id)?;
        let username = connected_user.username.clone();
        let current_game_name = connected_user.current_game_name.clone().ok_or(AppError {
            error_type: common::errors::AppErrorTypes::ServerError,
            message: error_messages::MISSING_GAME_REFERENCE.to_string(),
        })?;

        let game = get_mut_game(&mut self.games, &current_game_name)?;

        game.remove_player_from_adventuring_party(username.clone(), true);

        self.emit_packet(
            &current_game_name,
            &GameServerUpdatePackets::PlayerChangedAdventuringParty(PlayerAdventuringPartyChange {
                username: username.clone(),
                party_id: None,
            }),
            None,
        )?;

        Ok(())
    }
}
