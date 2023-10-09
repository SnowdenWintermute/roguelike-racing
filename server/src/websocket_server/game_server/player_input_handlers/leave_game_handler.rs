use crate::websocket_server::game_server::GameServer;
use common::{
    consts::MAIN_CHAT_ROOM,
    errors::AppError,
    game::{player_actions::GameCreation, RoguelikeRacerPlayer},
    packets::server_to_client::GameServerUpdatePackets,
};

use super::join_room_handler::join_room_handler;

pub fn leave_game_handler(game_server: &mut GameServer, actor_id: u32) {
    join_room_handler(game_server, MAIN_CHAT_ROOM, actor_id);
    if let Ok(player_and_game) = remove_player_from_game(game_server, actor_id) {
        game_server.emit_packet(
            player_and_game.game_name.as_str(),
            &GameServerUpdatePackets::UserLeftGame(player_and_game.username.clone()),
            Some(actor_id),
        );
        game_server.send_packet(&GameServerUpdatePackets::GameFullUpdate(None), actor_id);
        game_server.send_packet(
            &GameServerUpdatePackets::ClientAdventuringPartyId(None),
            actor_id,
        );
    } else {
        println!("error leaving game")
    }
}

pub struct PlayerRemovedFromGame {
    pub username: String,
    pub game_name: String,
}

pub fn remove_player_from_game(
    game_server: &mut GameServer,
    actor_id: u32,
) -> Result<PlayerRemovedFromGame, AppError> {
    let connected_user = match game_server.sessions.get_mut(&actor_id) {
        Some(user) => user,
        None => {
            println!("tried to leave game but no user was found");
            return Err(AppError {
                error_type: common::errors::AppErrorTypes::ServerError,
                message: "tried to leave game but no user was found".to_string(),
            });
        }
    };

    match &connected_user.current_game_name {
        Some(game_name) => {
            match game_server.games.get_mut(game_name) {
                Some(game) => {
                    // remove player from game
                    game.partyless_players
                        .remove(&connected_user.username.clone());
                    game.remove_player_from_adventuring_party(connected_user.username.clone());
                    // if game empty remove it
                    if game.get_number_of_players() < 1 {
                        game_server.games.remove(game_name);
                    }
                }
                None => {
                    println!("no game by that name was found");
                    return Err(AppError {
                        error_type: common::errors::AppErrorTypes::ServerError,
                        message: "no game by that name was found".to_string(),
                    });
                }
            }

            let game_name_leaving = game_name.clone();
            // remove game name from user's current game slot
            connected_user.current_game_name = None;
            Ok(PlayerRemovedFromGame {
                username: connected_user.username.clone(),
                game_name: game_name_leaving,
            })
        }

        None => {
            println!("can't leave a game if user isn't in one");
            return Err(AppError {
                error_type: common::errors::AppErrorTypes::ServerError,
                message: "can't leave a game if user isn't in one".to_string(),
            });
        }
    }
}
