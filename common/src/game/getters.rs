use crate::{
    adventuring_party::AdventuringParty,
    app_consts::error_messages,
    errors::{AppError, AppErrorTypes},
};

use super::{RoguelikeRacerGame, RoguelikeRacerPlayer};

pub fn get_mut_player<'a>(
    game: &'a mut RoguelikeRacerGame,
    username: String,
) -> Result<&'a mut RoguelikeRacerPlayer, AppError> {
    let player = game.players.get_mut(&username).ok_or_else(|| AppError {
        error_type: AppErrorTypes::ServerError,
        message: error_messages::PLAYER_NOT_FOUND.to_string(),
    })?;
    Ok(player)
}

pub fn get_mut_party<'a>(
    game: &'a mut RoguelikeRacerGame,
    party_id: u32,
) -> Result<&'a mut AdventuringParty, AppError> {
    let party = game
        .adventuring_parties
        .get_mut(&party_id)
        .ok_or_else(|| AppError {
            error_type: crate::errors::AppErrorTypes::ServerError,
            message: error_messages::PARTY_NOT_FOUND.to_string(),
        })?;
    Ok(party)
}
