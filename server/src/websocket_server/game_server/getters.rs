use super::ConnectedUser;
use common::{app_consts::error_messages, errors::AppError, game::RoguelikeRacerGame};
use std::collections::HashMap;

pub fn get_user<'a>(
    sessions: &'a HashMap<u32, ConnectedUser>,
    actor_id: u32,
) -> Result<&'a ConnectedUser, AppError> {
    let user = sessions.get(&actor_id).ok_or_else(|| AppError {
        error_type: common::errors::AppErrorTypes::ServerError,
        message: error_messages::USER_NOT_FOUND.to_string(),
    })?;
    Ok(user)
}

pub fn get_mut_user<'a>(
    sessions: &'a mut HashMap<u32, ConnectedUser>,
    actor_id: u32,
) -> Result<&'a mut ConnectedUser, AppError> {
    let user = sessions.get_mut(&actor_id).ok_or_else(|| AppError {
        error_type: common::errors::AppErrorTypes::ServerError,
        message: error_messages::USER_NOT_FOUND.to_string(),
    })?;
    Ok(user)
}

pub fn get_game<'a>(
    games: &'a HashMap<String, RoguelikeRacerGame>,
    game_name: String,
) -> Result<&'a RoguelikeRacerGame, AppError> {
    let game = games.get(&game_name).ok_or_else(|| AppError {
        error_type: common::errors::AppErrorTypes::ServerError,
        message: error_messages::GAME_NOT_FOUND.to_string(),
    })?;
    Ok(game)
}

pub fn get_mut_game<'a>(
    games: &'a mut HashMap<String, RoguelikeRacerGame>,
    game_name: &'a str,
) -> Result<&'a mut RoguelikeRacerGame, AppError> {
    let game = games.get_mut(game_name).ok_or_else(|| AppError {
        error_type: common::errors::AppErrorTypes::ServerError,
        message: error_messages::GAME_NOT_FOUND.to_string(),
    })?;
    Ok(game)
}
