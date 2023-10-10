use common::{
    errors::AppError,
    packets::server_to_client::{GameServerUpdatePackets, PlayerAdventuringPartyChange},
};

use crate::websocket_server::game_server::GameServer;

impl GameServer {
    pub fn leave_adventuring_party_handler(&mut self, actor_id: u32) -> Result<(), AppError> {
        let connected_user = match self.sessions.get_mut(&actor_id) {
            Some(user) => user,
            None => {
                println!("tried to leave party but no user was found");
                return Err(AppError {
                    error_type: common::errors::AppErrorTypes::ServerError,
                    message: "tried to leave party but no user was found".to_string(),
                });
            }
        };

        let username = connected_user.username.clone();
        if let Some(current_game_name) = connected_user.current_game_name.clone() {
            match self.games.get_mut(&current_game_name) {
                Some(game) => {
                    game.remove_player_from_adventuring_party(username.clone());
                }
                None => {
                    println!("no game by that name was found");
                    return Err(AppError {
                        error_type: common::errors::AppErrorTypes::ServerError,
                        message: "no game by that name was found".to_string(),
                    });
                }
            };
            self.emit_packet(&current_game_name, &GameServerUpdatePackets::PlayerChangedAdventuringParty(PlayerAdventuringPartyChange{
                username: username.clone(),
                party_id:None
            }), None);
            Ok(())
        } else {
            println!("playery tried to leave a party but they had no reference to a game name");
            return Err(AppError {
                error_type: common::errors::AppErrorTypes::ServerError,
                message: "player tried to leave a party but they had no reference to a game name"
                    .to_string(),
            })
        }
    }
}
