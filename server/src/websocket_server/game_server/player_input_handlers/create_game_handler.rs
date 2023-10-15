use crate::websocket_server::game_server::getters::get_mut_user;
use crate::websocket_server::game_server::GameServer;
use common::app_consts::error_messages;
use common::errors::AppError;
use common::game::RoguelikeRacerGame;
use common::packets::client_to_server::GameCreation;

impl GameServer {
    pub fn create_game_handler(
        &mut self,
        actor_id: u32,
        message_content: GameCreation,
    ) -> Result<(), AppError> {
        let GameCreation {
            name: game_name,
            password: _,
        } = &message_content;

        let connected_user = get_mut_user(&mut self.sessions, actor_id)?;
        if connected_user.current_game_name.is_some() {
            return Err(AppError {
                error_type: common::errors::AppErrorTypes::InvalidInput,
                message: error_messages::ALREADY_IN_GAME.to_string(),
            });
        }
        if self.games.get(game_name).is_some() {
            return Err(AppError {
                error_type: common::errors::AppErrorTypes::InvalidInput,
                message: error_messages::GAME_ALREADY_EXISTS.to_string(),
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
