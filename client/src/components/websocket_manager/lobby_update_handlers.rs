use crate::store::{game_store::GameStore, lobby_store::LobbyStore};
use common::{errors::AppError, game::RoguelikeRacerPlayer};

pub fn handle_user_left_room(lobby_state: &mut LobbyStore, username_leaving: &str) {
    for (index, username) in lobby_state.room.users.clone().iter().enumerate() {
        if username_leaving == username {
            lobby_state.room.users.remove(index);
        }
    }
}

pub fn handle_user_joined_game(
    game_state: &mut GameStore,
    username: String,
) -> Result<(), AppError> {
    let game = game_state.game.as_mut().ok_or_else(|| AppError {
        error_type: common::errors::AppErrorTypes::ClientError,
        message: "Client error".to_string(),
    })?;
    game.players
        .insert(username.clone(), RoguelikeRacerPlayer::new(None, username));
    Ok(())
}

pub fn handle_user_left_game(game_state: &mut GameStore, username: String) -> Result<(), AppError> {
    let game = game_state.game.as_mut().ok_or_else(|| AppError {
        error_type: common::errors::AppErrorTypes::ClientError,
        message: "Client error".to_string(),
    })?;
    let _ = &game.remove_player_from_adventuring_party(username.clone());
    game.players.remove(&username);
    game.players_readied.remove(&username);
    Ok(())
}

pub fn handle_player_toggled_ready(
    game_state: &mut GameStore,
    username: String,
) -> Result<(), AppError> {
    let game = game_state.game.as_mut().ok_or_else(|| AppError {
        error_type: common::errors::AppErrorTypes::ClientError,
        message: "Client error".to_string(),
    })?;
    if game.players_readied.contains(&username) {
        game.players_readied.remove(&username);
    } else {
        game.players_readied.insert(username.clone());
    }

    Ok(())
}

pub fn handle_game_started(game_state: &mut GameStore, timestamp: u128) -> Result<(), AppError> {
    let game = game_state.game.as_mut().ok_or_else(|| AppError {
        error_type: common::errors::AppErrorTypes::ClientError,
        message: "Client error".to_string(),
    })?;
    game.time_started = Some(timestamp);

    Ok(())
}
