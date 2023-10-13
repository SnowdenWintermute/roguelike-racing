use crate::{
    app_consts::error_messages,
    errors::{AppError, AppErrorTypes},
};

use super::{RoguelikeRacerGame, RoguelikeRacerPlayer};

pub fn get_mut_player<'a>(
    game: &'a mut RoguelikeRacerGame,
    username: String,
) -> Result<&'a mut RoguelikeRacerPlayer, AppError> {
    let player = game.players.get_mut(&username).ok_or(AppError {
        error_type: AppErrorTypes::ServerError,
        message: error_messages::PLAYER_NOT_FOUND.to_string(),
    })?;
    Ok(player)
}
