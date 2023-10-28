use common::{
    app_consts::error_messages, errors::AppError, game::getters::get_mut_player,
    packets::server_to_client::GameServerUpdatePackets,
};
use std::time::{SystemTime, UNIX_EPOCH};

use crate::websocket_server::game_server::{
    getters::{get_mut_game, get_mut_user},
    GameServer,
};

impl GameServer {
    pub fn toggle_ready_handler(&mut self, actor_id: u32) -> Result<(), AppError> {
        let user = get_mut_user(&mut self.sessions, actor_id)?;
        let username = user.username.clone();
        let game_name = user.current_game_name.clone().ok_or_else(|| AppError {
            error_type: common::errors::AppErrorTypes::ServerError,
            message: error_messages::MISSING_GAME_REFERENCE.to_string(),
        })?;
        let game = get_mut_game(&mut self.games, &game_name)?;
        if game.time_started.is_some() {
            return Err(AppError {
                error_type: common::errors::AppErrorTypes::InvalidInput,
                message: error_messages::GAME_HAS_STARTED.to_string(),
            });
        }
        let game_name = game.name.clone();

        // only allow readying if they have at least one character
        let player = get_mut_player(game, username.clone())?;
        let _ = player.character_ids.clone().ok_or_else(|| AppError {
            error_type: common::errors::AppErrorTypes::InvalidInput,
            message: error_messages::PLAYER_HAS_NO_CHARACTERS.to_string(),
        })?;

        // add or remove player's username to list of readied players
        if game.players_readied.contains(&username) {
            game.players_readied.remove(&username);
        } else {
            game.players_readied.insert(username.clone());
        }

        let all_players_ready = (|| -> bool {
            for (username, _) in &game.players {
                if game.players_readied.contains(username) {
                    continue;
                }
                return false;
            }
            true
        })();

        println!("all players readied: {}", all_players_ready);
        // if all players have their name in the readied list, start the game
        if all_players_ready {
            game.time_started = Some(
                SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .expect("time went backwards")
                    .as_millis(),
            );

            let time_started = game.time_started.expect("was just set");
            self.emit_packet(
                &game_name,
                &GameServerUpdatePackets::GameStarted(time_started),
                None,
            )?;
        }

        self.emit_packet(
            &game_name,
            &GameServerUpdatePackets::PlayerToggledReady(username),
            None,
        )?;

        Ok(())
    }
}
