use super::RoguelikeRacerGame;
use super::RoguelikeRacerPlayer;
use crate::adventuring_party::AdventuringParty;
use crate::app_consts::error_messages;
use crate::character::Character;
use crate::combat::battle::Battle;
use crate::errors::AppError;
use crate::errors::AppErrorTypes;

pub fn get_mut_player<'a>(
    game: &'a mut RoguelikeRacerGame,
    username: &String,
) -> Result<&'a mut RoguelikeRacerPlayer, AppError> {
    let player = game.players.get_mut(username).ok_or_else(|| AppError {
        error_type: AppErrorTypes::ServerError,
        message: error_messages::PLAYER_NOT_FOUND.to_string(),
    })?;
    Ok(player)
}

pub fn get_player<'a>(
    game: &'a RoguelikeRacerGame,
    username: String,
) -> Result<&'a RoguelikeRacerPlayer, AppError> {
    let player = game.players.get(&username).ok_or_else(|| AppError {
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

pub fn get_party<'a>(
    game: &'a RoguelikeRacerGame,
    party_id: u32,
) -> Result<&'a AdventuringParty, AppError> {
    let party = game
        .adventuring_parties
        .get(&party_id)
        .ok_or_else(|| AppError {
            error_type: crate::errors::AppErrorTypes::ServerError,
            message: error_messages::PARTY_NOT_FOUND.to_string(),
        })?;
    Ok(party)
}

pub fn get_mut_character<'a>(
    game: &'a mut RoguelikeRacerGame,
    party_id: u32,
    character_id: u32,
) -> Result<&'a mut Character, AppError> {
    let party = get_mut_party(game, party_id)?;
    let character = party
        .characters
        .get_mut(&character_id)
        .ok_or_else(|| AppError {
            error_type: crate::errors::AppErrorTypes::ServerError,
            message: error_messages::CHARACTER_NOT_FOUND.to_string(),
        })?;
    Ok(character)
}

pub fn get_character<'a>(
    game: &'a RoguelikeRacerGame,
    party_id: u32,
    character_id: u32,
) -> Result<&'a Character, AppError> {
    let party = get_party(game, party_id)?;
    let character = party
        .characters
        .get(&character_id)
        .ok_or_else(|| AppError {
            error_type: crate::errors::AppErrorTypes::ServerError,
            message: error_messages::CHARACTER_NOT_FOUND.to_string(),
        })?;
    Ok(character)
}

pub fn get_ally_ids_and_opponent_ids_option(
    ally_ids: &Vec<u32>,
    battle_option: Option<&Battle>,
    combatant_id: u32,
) -> Result<(Vec<u32>, Option<Vec<u32>>), AppError> {
    if let Some(battle) = battle_option {
        battle.get_ally_ids_and_opponent_ids_option(combatant_id)
    } else {
        Ok((ally_ids.to_vec(), None))
    }
}
