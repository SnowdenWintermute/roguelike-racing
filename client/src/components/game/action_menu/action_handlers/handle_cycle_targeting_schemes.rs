use crate::{
    components::websocket_manager::send_client_input::send_client_input,
    store::game_store::GameStore,
};
use common::{
    app_consts::error_messages,
    errors::AppError,
    game::getters::get_mut_party,
    packets::client_to_server::{ChangeTargetsPacket, PlayerInputs},
};
use gloo::console::log;
use web_sys::WebSocket;

pub fn handle_cycle_targeting_schemes(
    game_store: &mut GameStore,
    websocket_option: &Option<WebSocket>,
) {
    let mut closure = move || -> Result<(), AppError> {
        let game = game_store.game.as_mut().ok_or_else(|| AppError {
            error_type: common::errors::AppErrorTypes::ClientError,
            message: error_messages::MISSING_GAME_REFERENCE.to_string(),
        })?;
        let party_id = game_store.current_party_id.ok_or_else(|| AppError {
            error_type: common::errors::AppErrorTypes::ClientError,
            message: error_messages::MISSING_PARTY_REFERENCE.to_string(),
        })?;
        let party = get_mut_party(game, party_id)?;
        let focused_character = party
            .characters
            .get(&game_store.focused_character_id)
            .ok_or_else(|| AppError {
                error_type: common::errors::AppErrorTypes::ClientError,
                message: error_messages::CHARACTER_NOT_FOUND.to_string(),
            })?;
        let ability_name = focused_character
            .combatant_properties
            .selected_ability_name
            .as_ref()
            .ok_or_else(|| AppError {
                error_type: common::errors::AppErrorTypes::ClientError,
                message: error_messages::MISSING_ABILITY_REFERENCE.to_string(),
            })?;
        // if only one targeting scheme, return early
        let ability_attributes = ability_name.get_attributes();
        if ability_attributes.targeting_schemes.len() < 2 {
            return Ok(());
        }

        let last_used_targeting_scheme = &focused_character
            .combatant_properties
            .ability_target_preferences
            .targeting_scheme_preference;

        let new_targeting_scheme = if !ability_attributes
            .targeting_schemes
            .contains(last_used_targeting_scheme)
        {
            ability_attributes
                .targeting_schemes
                .first()
                .ok_or_else(|| AppError {
                    error_type: common::errors::AppErrorTypes::ClientError,
                    message: error_messages::MISSING_ABILITY_REFERENCE.to_string(),
                })?
        } else {
            let last_used_targeting_scheme_index = ability_attributes
                .targeting_schemes
                .iter()
                .position(|scheme| scheme == last_used_targeting_scheme)
                .expect("checked it contains above");
            let new_scheme_index = if last_used_targeting_scheme_index
                == ability_attributes.targeting_schemes.len() - 1
            {
                0
            } else {
                last_used_targeting_scheme_index + 1
            };
            ability_attributes
                .targeting_schemes
                .get(new_scheme_index)
                .expect("a valid index")
        };

        let focused_character = party
            .characters
            .get_mut(&game_store.focused_character_id)
            .ok_or_else(|| AppError {
                error_type: common::errors::AppErrorTypes::ClientError,
                message: error_messages::CHARACTER_NOT_FOUND.to_string(),
            })?;

        focused_character
            .combatant_properties
            .ability_target_preferences
            .targeting_scheme_preference = new_targeting_scheme.clone();

        let focused_character = party
            .characters
            .get(&game_store.focused_character_id)
            .ok_or_else(|| AppError {
                error_type: common::errors::AppErrorTypes::ClientError,
                message: error_messages::CHARACTER_NOT_FOUND.to_string(),
            })?;
        let ability_name = focused_character
            .combatant_properties
            .selected_ability_name
            .as_ref()
            .ok_or_else(|| AppError {
                error_type: common::errors::AppErrorTypes::ClientError,
                message: error_messages::MISSING_ABILITY_REFERENCE.to_string(),
            })?;
        let new_targets = ability_name.targets_by_saved_preference_or_default(
            focused_character.entity_properties.id,
            &focused_character
                .combatant_properties
                .ability_target_preferences,
            party,
        )?;

        let new_preferences = focused_character
            .combatant_properties
            .ability_target_preferences
            .get_updated_preferences(ability_name, &new_targets, party);
        let focused_character = party
            .characters
            .get_mut(&game_store.focused_character_id)
            .ok_or_else(|| AppError {
                error_type: common::errors::AppErrorTypes::ClientError,
                message: error_messages::CHARACTER_NOT_FOUND.to_string(),
            })?;
        focused_character
            .combatant_properties
            .ability_target_preferences = new_preferences;

        send_client_input(
            &websocket_option,
            PlayerInputs::ChangeTargets(ChangeTargetsPacket {
                character_id: focused_character.entity_properties.id,
                new_targets,
            }),
        );
        Ok(())
    };
    let result = closure();
    if result.is_ok() {
        ()
    } else {
        log!("an unhandled client error occured");
    }
}
