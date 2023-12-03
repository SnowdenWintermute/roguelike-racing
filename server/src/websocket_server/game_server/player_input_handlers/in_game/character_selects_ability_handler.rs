use crate::websocket_server::game_server::{
    getters::{get_mut_party_game_name_and_character_ids_from_actor_id, ActorIdAssociatedGameData},
    GameServer,
};
use common::{
    app_consts::error_messages, errors::AppError,
    packets::client_to_server::ClientSelectAbilityPacket,
};

impl GameServer {
    pub fn character_selects_ability_handler(
        &mut self,
        actor_id: u32,
        packet: ClientSelectAbilityPacket,
    ) -> Result<(), AppError> {
        let ActorIdAssociatedGameData {
            party,
            current_game_name,
            username,
            player_character_ids_option,
            ..
        } = get_mut_party_game_name_and_character_ids_from_actor_id(self, actor_id)?;

        let player_character_ids = player_character_ids_option.ok_or_else(|| AppError {
            error_type: common::errors::AppErrorTypes::ServerError,
            message: error_messages::PLAYER_HAS_NO_CHARACTERS.to_string(),
        })?;
        let character = match player_character_ids.contains(&packet.character_id) {
            true => party
                .characters
                .get(&packet.character_id)
                .ok_or_else(|| AppError {
                    error_type: common::errors::AppErrorTypes::ServerError,
                    message: error_messages::CHARACTER_NOT_FOUND.to_string(),
                }),
            false => Err(AppError {
                error_type: common::errors::AppErrorTypes::ServerError,
                message: error_messages::CHARACTER_NOT_OWNED.to_string(),
            }),
        }?;

        let new_target_ids = match &packet.ability_name_option {
            Some(ability_name) => {
                // check if ability is valid
                let ability = character
                    .combatant_properties
                    .abilities
                    .get(&ability_name)
                    .ok_or_else(|| AppError {
                        error_type: common::errors::AppErrorTypes::InvalidInput,
                        message: error_messages::ABILITY_NOT_OWNED.to_string(),
                    })?;
                let previous_targets_are_still_valid = ability.last_targets_are_still_valid(&party);

                let new_target_ids = if previous_targets_are_still_valid {
                    ability.most_recently_targeted.clone()
                } else {
                    ability
                        .get_default_target_ids(&party, packet.character_id)
                        .clone()
                };
                new_target_ids
            }
            None => None,
        };

        let character = match player_character_ids.contains(&packet.character_id) {
            true => party
                .characters
                .get_mut(&packet.character_id)
                .ok_or_else(|| AppError {
                    error_type: common::errors::AppErrorTypes::ServerError,
                    message: error_messages::CHARACTER_NOT_FOUND.to_string(),
                }),
            false => Err(AppError {
                error_type: common::errors::AppErrorTypes::ServerError,
                message: error_messages::CHARACTER_NOT_OWNED.to_string(),
            }),
        }?;

        // set the new targets
        character.combatant_properties.ability_target_ids = new_target_ids.clone();
        // set the new ability selected
        character.combatant_properties.selected_ability_name = packet.ability_name_option.clone();
        // save prev targets
        if let Some(ability_name) = &packet.ability_name_option {
            let ability = character
                .combatant_properties
                .abilities
                .get_mut(&ability_name)
                .ok_or_else(|| AppError {
                    error_type: common::errors::AppErrorTypes::InvalidInput,
                    message: error_messages::ABILITY_NOT_OWNED.to_string(),
                })?;

            ability.most_recently_targeted = new_target_ids.clone();
        }

        //
        // emit to party the changes
        Ok(())
    }
}
