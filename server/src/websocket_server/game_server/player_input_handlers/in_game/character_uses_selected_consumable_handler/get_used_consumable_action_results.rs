use crate::websocket_server::game_server::getters::get_game;
use crate::websocket_server::game_server::getters::get_user;
use crate::websocket_server::game_server::GameServer;
use common::app_consts::error_messages;
use common::combat::ActionResult;
use common::errors::AppError;
use common::game::getters::get_party;
use common::game::getters::get_player;

impl GameServer {
    pub fn get_used_consumable_action_results(
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
        let consumable_id = character
            .combatant_properties
            .selected_consumable
            .ok_or_else(|| AppError {
                error_type: common::errors::AppErrorTypes::Generic,
                message: error_messages::NO_CONSUMABLE_SELECTED.to_string(),
            })?;
        let consumable = character.inventory.get_consumable(&consumable_id)?;
        let combat_action_properties = consumable.consumable_type.get_combat_action_properties();

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

        combat_action_properties.validate_use(
            battle_option.as_ref(),
            &ally_ids,
            &targets,
            character_id,
        )?;

        game.get_consumable_use_results(
            party_id,
            character_id,
            consumable_id,
            &targets,
            battle_option.as_ref(),
        )
    }
}
