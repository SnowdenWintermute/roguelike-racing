use crate::utils::generate_random_character_name;
use crate::websocket_server::game_server::getters::get_mut_game;
use crate::websocket_server::game_server::getters::get_user;
use crate::websocket_server::game_server::GameServer;
use common::errors::AppError;
use common::game::getters::get_mut_character;
use common::game::getters::get_mut_player;
use common::packets::client_to_server::CharacterCreation;
use common::packets::server_to_client::GameServerUpdatePackets;
use common::packets::WebsocketChannelNamespace;
use std::collections::HashSet;

impl GameServer {
    pub fn create_character_handler(
        &mut self,
        actor_id: u32,
        character_creation: CharacterCreation,
    ) -> Result<(), AppError> {
        let user = get_user(&self.sessions, actor_id)?;
        let username = user.username.clone();
        let current_game_name = user.current_game_name.clone().ok_or_else(|| AppError {
            error_type: common::errors::AppErrorTypes::ServerError,
            message: common::app_consts::error_messages::MISSING_GAME_REFERENCE.to_string(),
        })?;
        let game = get_mut_game(&mut self.games, &current_game_name)?;
        let game_name = game.name.clone();

        let player = get_mut_player(game, &user.username)?;
        let party_id = player.party_id.ok_or_else(|| AppError {
            error_type: common::errors::AppErrorTypes::ServerError,
            message: common::app_consts::error_messages::MISSING_PARTY_REFERENCE.to_string(),
        })?;

        let mut character_name = character_creation.character_name;
        if character_name == "" {
            character_name = generate_random_character_name();
        }

        let new_character_id = game.add_character_to_adventuring_party(
            party_id,
            character_creation.combatant_class.clone(),
            &character_name,
            username.clone(),
        )?;

        let player = get_mut_player(game, &user.username)?;
        match &mut player.character_ids {
            None => {
                let mut new_ids = HashSet::new();
                new_ids.insert(new_character_id);
                player.character_ids = Some(new_ids);
            }
            Some(ids) => {
                ids.insert(new_character_id);
            }
        }

        let character = get_mut_character(game, party_id, new_character_id)?;
        character.combatant_properties.get_total_attributes();
        let cloned_character = character.clone();

        self.emit_packet(
            &game_name,
            &WebsocketChannelNamespace::Game,
            &GameServerUpdatePackets::CharacterCreation(
                common::packets::server_to_client::NewCharacterInParty {
                    party_id,
                    username,
                    character: cloned_character,
                },
            ),
            None,
        )?;

        Ok(())
    }
}
