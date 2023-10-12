use crate::websocket_server::game_server::{get_mut_user, GameServer};
use common::errors::AppError;
use common::game::player_actions::{GameCreation, PlayerInputs};
use common::game::{RoguelikeRacerGame, RoguelikeRacerPlayer};

impl GameServer {
    pub fn create_game_handler(
        &mut self,
        actor_id: u32,
        message_content: GameCreation,
    ) -> Result<(), AppError> {
        println!("game creation request received");
        let GameCreation {
            name: game_name,
            password: _,
        } = &message_content;

        let connected_user = get_mut_user(&mut self.sessions, actor_id)?;
        if connected_user.current_game_name.is_some() {
            return Err(AppError {
                error_type: common::errors::AppErrorTypes::ServerError,
                message: "leave your current game before creating one".to_string(),
            });
        }
        if self.games.get(game_name).is_some() {
            return Err(AppError {
                error_type: common::errors::AppErrorTypes::ServerError,
                message: "a game with name {} already exists".to_string(),
            });
        }

        // create the game and register it with the game_server
        self.games.insert(
            game_name.to_string(),
            RoguelikeRacerGame::new(game_name.to_string()),
        );
        let actor_id = connected_user.id;
        self.join_game_handler(actor_id, game_name.to_string())?;
        Ok(())
    }
}
