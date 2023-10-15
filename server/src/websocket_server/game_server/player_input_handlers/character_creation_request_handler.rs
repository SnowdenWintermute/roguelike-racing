use crate::websocket_server::game_server::{
    getters::{get_mut_game, get_user},
    GameServer,
};
use common::{
    app_consts::MAX_PARTY_SIZE,
    errors::AppError,
    game::getters::{get_mut_party, get_mut_player},
    packets::client_to_server::CharacterCreation,
};

impl GameServer {
    pub fn character_creation_request_handler(
        &mut self,
        actor_id: u32,
        character_creation: CharacterCreation,
    ) -> Result<(), AppError> {
        let user = get_user(&self.sessions, actor_id)?;
        let current_game_name = user.current_game_name.clone().ok_or_else(|| AppError {
            error_type: common::errors::AppErrorTypes::ServerError,
            message: common::app_consts::error_messages::MISSING_GAME_REFERENCE.to_string(),
        })?;
        let game = get_mut_game(&mut self.games, &current_game_name)?;
        let player = get_mut_player(game, user.username.clone())?;
        let party_id = player.party_id.ok_or_else(|| AppError {
            error_type: common::errors::AppErrorTypes::ServerError,
            message: common::app_consts::error_messages::MISSING_PARTY_REFERENCE.to_string(),
        })?;
        let party = get_mut_party(game, party_id)?;

        if party.player_characters.len() as u8 > MAX_PARTY_SIZE {
            return Err(AppError {
                error_type: common::errors::AppErrorTypes::InvalidInput,
                message: common::app_consts::error_messages::PARTY_CHARACTER_LIMIT_REACHED
                    .to_string(),
            });
        }

        // create a character object
        // give it the id of it's player
        // give the player a reference to the character
        // update the game room

        Ok(())
    }
}
