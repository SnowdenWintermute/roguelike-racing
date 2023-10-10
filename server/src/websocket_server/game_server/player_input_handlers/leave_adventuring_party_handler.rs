use common::errors::AppError;

use crate::websocket_server::game_server::GameServer;

impl GameServer {
    pub fn leave_adventuring_party_handler(&mut self, actor_id: u32) {
        let connected_user = match game_server.sessions.get_mut(&actor_id) {
            Some(user) => user,
            None => {
                println!("tried to leave party but no user was found");
                return Err(AppError {
                    error_type: common::errors::AppErrorTypes::ServerError,
                    message: "tried to leave party but no user was found".to_string(),
                });
            }
        };
        match &connected_user.current_game_name {
            Some(game_name) => match game_server.games.get_mut(game_name) {
                Some(game) => {
                    game.remove_player_from_adventuring_party(connected_user.username.clone());
                }
                None => {
                    println!("no game by that name was found");
                    return Err(AppError {
                        error_type: common::errors::AppErrorTypes::ServerError,
                        message: "no game by that name was found".to_string(),
                    });
                }
            },
            None => {
                println!("playery tried to leave a party but they had no reference to a game name")
            }
        }
    }
}
