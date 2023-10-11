use crate::websocket_server::game_server::GameServer;
use common::errors::AppError;
use common::packets::server_to_client::{GameServerUpdatePackets, PlayerAdventuringPartyChange};

impl GameServer {
    pub fn leave_adventuring_party_handler(&mut self, actor_id: u32) -> Result<(), AppError> {
        let connected_user = self.sessions.get_mut(&actor_id).ok_or(AppError {
            error_type: common::errors::AppErrorTypes::ServerError,
            message: "No user found".to_string(),
        })?;

        let username = connected_user.username.clone();
        let current_game_name = connected_user.current_game_name.clone().ok_or(AppError {
            error_type: common::errors::AppErrorTypes::ServerError,
            message: "Missing reference to current game".to_string(),
        })?;

        let game = self.games.get_mut(&current_game_name).ok_or(AppError {
            error_type: common::errors::AppErrorTypes::ServerError,
            message: "No game found".to_string(),
        })?;

        game.remove_player_from_adventuring_party(username.clone(), true);

        self.emit_packet(
            &current_game_name,
            &GameServerUpdatePackets::PlayerChangedAdventuringParty(PlayerAdventuringPartyChange {
                username: username.clone(),
                party_id: None,
            }),
            None,
        );

        Ok(())
    }
}
