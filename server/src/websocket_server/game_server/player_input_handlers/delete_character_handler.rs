use common::{
    app_consts::error_messages,
    errors::AppError,
    game::getters::{get_mut_party, get_mut_player},
    packets::server_to_client::{GameServerUpdatePackets, PlayerCharacterDeletion},
};

use crate::websocket_server::game_server::{
    getters::{get_mut_game, get_mut_user},
    GameServer,
};

impl GameServer {
    pub fn delete_character_handler(
        &mut self,
        actor_id: u32,
        character_id: u32,
    ) -> Result<(), AppError> {
        let connected_user = get_mut_user(&mut self.sessions, actor_id)?;
        let username = connected_user.username.clone();
        let current_game_name =
            connected_user
                .current_game_name
                .clone()
                .ok_or_else(|| AppError {
                    error_type: common::errors::AppErrorTypes::ServerError,
                    message: error_messages::MISSING_GAME_REFERENCE.to_string(),
                })?;
        let game = get_mut_game(&mut self.games, &current_game_name)?;
        let game_name = game.name.clone();
        let player = get_mut_player(game, &username)?;
        let player_character_ids_option = player.character_ids.clone();
        let party_id = player.party_id.ok_or_else(|| AppError {
            error_type: common::errors::AppErrorTypes::ServerError,
            message: error_messages::MISSING_PARTY_REFERENCE.to_string(),
        })?;

        let party = get_mut_party(game, party_id)?;
        let mut player_character_ids = player_character_ids_option.ok_or_else(|| AppError {
            error_type: common::errors::AppErrorTypes::ServerError,
            message: error_messages::PLAYER_HAS_NO_CHARACTERS.to_string(),
        })?;

        let mut should_unready_player = false;
        if player_character_ids.contains(&character_id) {
            party.remove_character(character_id);
            player_character_ids.remove(&character_id);

            let player = get_mut_player(game, &username)?;
            if player_character_ids.len() >= 1 {
                player.character_ids = Some(player_character_ids);
            } else {
                should_unready_player = true;
                player.character_ids = None
            }
        } else {
            return Err(AppError {
                error_type: common::errors::AppErrorTypes::ServerError,
                message: error_messages::CHARACTER_NOT_OWNED.to_string(),
            });
        }

        if should_unready_player {
            let was_ready = game.players_readied.remove(&username);
            if was_ready {
                self.emit_packet(
                    &game_name,
                    &GameServerUpdatePackets::PlayerToggledReady(username.clone()),
                    None,
                )?;
            }
        }

        self.emit_packet(
            &game_name,
            &GameServerUpdatePackets::CharacterDeletion(PlayerCharacterDeletion {
                party_id,
                character_id,
                username,
            }),
            None,
        )
    }
}
