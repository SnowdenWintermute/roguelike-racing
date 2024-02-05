use crate::websocket_server::game_server::getters::get_game;
use crate::websocket_server::game_server::getters::get_user;
use crate::websocket_server::game_server::GameServer;
use common::app_consts::error_messages;
use common::combat::ActionResult;
use common::errors::AppError;
use common::game::getters::get_party;
use common::game::getters::get_player;

impl GameServer {
    pub fn get_ability_action_results(
        &self,
        actor_id: u32,
        character_id: u32,
    ) -> Result<Vec<ActionResult>, AppError> {
        let connected_user = get_user(&self.sessions, actor_id)?;
        let username = connected_user.username.clone();
        let current_game_name =
            connected_user
                .current_game_name
                .as_ref()
                .ok_or_else(|| AppError {
                    error_type: common::errors::AppErrorTypes::ServerError,
                    message: error_messages::MISSING_GAME_REFERENCE.to_string(),
                })?;
        let game = get_game(&self.games, current_game_name.clone())?;
        let player = get_player(game, username)?;
        let player_character_ids_option = player.character_ids.clone();
        let party_id = player.party_id.ok_or_else(|| AppError {
            error_type: common::errors::AppErrorTypes::ServerError,
            message: error_messages::MISSING_PARTY_REFERENCE.to_string(),
        })?;

        let party_id = party_id.clone();
        let party = get_party(game, party_id)?;
        let battle_id_option = party.battle_id;
        let character_positions = party.character_positions.clone();
        let character = party.get_character_if_owned(player_character_ids_option, character_id)?;
        let ability_name = character
            .combatant_properties
            .selected_ability_name
            .clone()
            .ok_or_else(|| AppError {
                error_type: common::errors::AppErrorTypes::InvalidInput,
                message: error_messages::NO_ABILITY_SELECTED.to_string(),
            })?;
        let ability_attributes = ability_name.get_attributes();
        // check if they own the ability
        let _ = character
            .combatant_properties
            .get_ability_if_owned(&ability_name)?;

        let targets = character
            .combatant_properties
            .combat_action_targets
            .clone()
            .ok_or_else(|| AppError {
                error_type: common::errors::AppErrorTypes::InvalidInput,
                message: error_messages::NO_POSSIBLE_TARGETS_PROVIDED.to_string(),
            })?;

        let battle_option = if let Some(battle_id) = battle_id_option {
            Some(
                game.battles
                    .get(&battle_id)
                    .ok_or_else(|| AppError {
                        error_type: common::errors::AppErrorTypes::ServerError,
                        message: error_messages::BATTLE_NOT_FOUND.to_string(),
                    })?
                    .clone(),
            )
        } else {
            None
        };

        let ally_ids = if let Some(battle) = &battle_option {
            let (ally_ids, _) = battle.get_ally_ids_and_opponent_ids_option(character_id)?;
            ally_ids
        } else {
            character_positions.clone()
        };

        ability_attributes.combat_action_properties.validate_use(
            battle_option.as_ref(),
            &ally_ids,
            &targets,
            character_id,
        )?;

        game.get_ability_results(
            character_id,
            &ability_name,
            &targets,
            battle_option.as_ref(),
        )
    }
}
