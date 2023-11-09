use std::collections::HashSet;

use crate::websocket_server::game_server::{
    getters::{get_mut_game, get_user},
    GameServer,
};
use common::{
    errors::AppError,
    game::getters::{get_mut_party, get_mut_player},
    packets::{client_to_server::CharacterCreation, server_to_client::GameServerUpdatePackets},
};

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
        let next_entity_id = game.id_generator.get_next_entity_id();
        let game_name = game.name.clone();

        let player = get_mut_player(game, user.username.clone())?;
        let party_id = player.party_id.ok_or_else(|| AppError {
            error_type: common::errors::AppErrorTypes::ServerError,
            message: common::app_consts::error_messages::MISSING_PARTY_REFERENCE.to_string(),
        })?;

        game.add_character_to_adventuring_party(
            party_id,
            character_creation.combatant_class.clone(),
            &character_creation.character_name,
            username.clone(),
        )?;

        let player = get_mut_player(game, user.username.clone())?;
        match &mut player.character_ids {
            None => {
                let mut new_ids = HashSet::new();
                new_ids.insert(next_entity_id);
                player.character_ids = Some(new_ids);
            }
            Some(ids) => {
                ids.insert(next_entity_id);
            }
        }

        println!("{:#?}", game);
        self.emit_packet(
            &game_name,
            &GameServerUpdatePackets::CharacterCreation(
                common::packets::server_to_client::NewCharacterInParty {
                    party_id,
                    username,
                    character_id: next_entity_id,
                    character_name: character_creation.character_name.clone(),
                    combatant_class: character_creation.combatant_class,
                },
            ),
            None,
        )?;

        Ok(())
    }
}
