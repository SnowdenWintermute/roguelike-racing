use crate::components::websocket_manager::send_client_input::send_client_input;
use crate::store::game_store::get_cloned_current_battle_option;
use crate::store::game_store::GameStore;
use common::app_consts::error_messages;
use common::errors::AppError;
use common::game::getters::get_ally_ids_and_opponent_ids_option;
use common::game::getters::get_mut_party;
use common::packets::client_to_server::ChangeTargetsPacket;
use common::packets::client_to_server::PlayerInputs;
use gloo::console::log;
use web_sys::WebSocket;

pub fn handle_cycle_targeting_schemes(
    game_store: &mut GameStore,
    websocket_option: &Option<WebSocket>,
) {
    let mut closure = move || -> Result<(), AppError> {
        let battle_option = get_cloned_current_battle_option(&game_store);
        let game = game_store.game.as_mut().ok_or_else(|| AppError {
            error_type: common::errors::AppErrorTypes::ClientError,
            message: error_messages::MISSING_GAME_REFERENCE.to_string(),
        })?;
        let party_id = game_store.current_party_id.ok_or_else(|| AppError {
            error_type: common::errors::AppErrorTypes::ClientError,
            message: error_messages::MISSING_PARTY_REFERENCE.to_string(),
        })?;
        let party = get_mut_party(game, party_id)?;
        let cloned_character_positions = party.character_positions.clone();
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
        let combat_action_properties = ability_attributes.combat_action_properties;
        if combat_action_properties.targeting_schemes.len() < 2 {
            return Ok(());
        }

        let last_used_targeting_scheme = &focused_character
            .combatant_properties
            .ability_target_preferences
            .targeting_scheme_preference;

        let new_targeting_scheme = if !combat_action_properties
            .targeting_schemes
            .contains(last_used_targeting_scheme)
        {
            combat_action_properties
                .targeting_schemes
                .first()
                .ok_or_else(|| AppError {
                    error_type: common::errors::AppErrorTypes::ClientError,
                    message: error_messages::MISSING_ABILITY_REFERENCE.to_string(),
                })?
        } else {
            let last_used_targeting_scheme_index = combat_action_properties
                .targeting_schemes
                .iter()
                .position(|scheme| scheme == last_used_targeting_scheme)
                .expect("checked it contains above");
            let new_scheme_index = if last_used_targeting_scheme_index
                == combat_action_properties.targeting_schemes.len() - 1
            {
                0
            } else {
                last_used_targeting_scheme_index + 1
            };
            combat_action_properties
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

        let (ally_ids, opponent_ids_option) = get_ally_ids_and_opponent_ids_option(
            &cloned_character_positions,
            battle_option.as_ref(),
            focused_character.entity_properties.id,
        )?;

        let new_targets = combat_action_properties.targets_by_saved_preference_or_default(
            focused_character.entity_properties.id,
            &focused_character
                .combatant_properties
                .ability_target_preferences,
            ally_ids.clone(),
            opponent_ids_option.clone(),
        )?;

        let new_preferences = focused_character
            .combatant_properties
            .ability_target_preferences
            .get_updated_preferences(
                &combat_action_properties,
                &new_targets,
                ally_ids,
                opponent_ids_option,
            );
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
