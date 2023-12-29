use super::{ConnectedUser, GameServer};
use common::{
    adventuring_party::AdventuringParty,
    app_consts::error_messages,
    errors::AppError,
    game::{
        getters::{get_mut_party, get_mut_player},
        RoguelikeRacerGame,
    },
};
use std::collections::{HashMap, HashSet};

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

pub struct ActorIdAssociatedPartyData<'a> {
    pub party: &'a mut AdventuringParty,
    pub party_id: u32,
    pub current_game_name: String,
    pub username: String,
    pub player_character_ids_option: Option<HashSet<u32>>,
}

pub fn get_mut_party_game_name_and_character_ids_from_actor_id<'a>(
    game_server: &'a mut GameServer,
    actor_id: u32,
) -> Result<ActorIdAssociatedPartyData, AppError> {
    let connected_user = get_mut_user(&mut game_server.sessions, actor_id)?;
    let username = connected_user.username.clone();
    let current_game_name = connected_user
        .current_game_name
        .as_ref()
        .ok_or_else(|| AppError {
            error_type: common::errors::AppErrorTypes::ServerError,
            message: error_messages::MISSING_GAME_REFERENCE.to_string(),
        })?;
    let game = get_mut_game(&mut game_server.games, &current_game_name)?;
    let player = get_mut_player(game, &username)?;
    let player_character_ids_option = player.character_ids.clone();
    let party_id = player.party_id.ok_or_else(|| AppError {
        error_type: common::errors::AppErrorTypes::ServerError,
        message: error_messages::MISSING_PARTY_REFERENCE.to_string(),
    })?;
    let party = get_mut_party(game, party_id)?;

    Ok(ActorIdAssociatedPartyData {
        party,
        party_id,
        current_game_name: current_game_name.clone(),
        username,
        player_character_ids_option,
    })
}

pub struct ActorIdAssociatedGameData<'a> {
    pub game: &'a mut RoguelikeRacerGame,
    pub party_id: u32,
    pub current_game_name: String,
    pub username: String,
    pub player_character_ids_option: Option<HashSet<u32>>,
}

pub fn get_mut_game_data_from_actor_id<'a>(
    game_server: &'a mut GameServer,
    actor_id: u32,
) -> Result<ActorIdAssociatedGameData, AppError> {
    let connected_user = get_user(&game_server.sessions, actor_id)?;
    let username = connected_user.username.clone();
    let current_game_name = connected_user
        .current_game_name
        .as_ref()
        .ok_or_else(|| AppError {
            error_type: common::errors::AppErrorTypes::ServerError,
            message: error_messages::MISSING_GAME_REFERENCE.to_string(),
        })?;
    let game = get_mut_game(&mut game_server.games, &current_game_name)?;
    let player = get_mut_player(game, &username)?;
    let player_character_ids_option = player.character_ids.clone();
    let party_id = player.party_id.ok_or_else(|| AppError {
        error_type: common::errors::AppErrorTypes::ServerError,
        message: error_messages::MISSING_PARTY_REFERENCE.to_string(),
    })?;

    Ok(ActorIdAssociatedGameData {
        game,
        party_id,
        current_game_name: current_game_name.clone(),
        username,
        player_character_ids_option,
    })
}
