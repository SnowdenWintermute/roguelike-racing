use common::{
    app_consts::error_messages,
    errors::AppError,
    game::getters::{get_mut_party, get_mut_player},
    packets::server_to_client::GameServerUpdatePackets,
};

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
        let game_name = game.name.clone();

        // only allow readying if they have at least one character
        let player = get_mut_player(game, username.clone())?;
        let _ = player.character_ids.clone().ok_or_else(|| AppError {
            error_type: common::errors::AppErrorTypes::ServerError,
            message: error_messages::PLAYER_HAS_NO_CHARACTERS.to_string(),
        })?;

        // add or remove player's username to list of readied players
        if game.players_readied.contains(&username) {
            game.players_readied.remove(&username);
        } else {
            game.players_readied.insert(username.clone());
        }

        self.emit_packet(
            &game_name,
            &GameServerUpdatePackets::PlayerToggledReady(username),
            None,
        )?;

        // let all_players_ready = (|| -> bool {

        // })();

        // if all players have their name in the readied list, start the game
        Ok(())
    }
}
