use crate::{
    components::websocket_manager::send_client_input::send_client_input,
    store::game_store::GameStore,
};
use common::{
    app_consts::error_messages,
    errors::AppError,
    game::getters::get_mut_party,
    packets::client_to_server::{ChangeTargetsPacket, PlayerInputs},
    primatives::NextOrPrevious,
};
use gloo::console::log;
use web_sys::WebSocket;
use yewdux::prelude::Dispatch;

pub fn handle_cycle_targets(
    game_dispatch: Dispatch<GameStore>,
    websocket_option: &Option<WebSocket>,
    direction: &NextOrPrevious,
) {
    game_dispatch.reduce_mut(|game_store| {
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
            let focused_character_id = focused_character.entity_properties.id;
            let ability_name = focused_character
                .combatant_properties
                .selected_ability_name
                .as_ref()
                .ok_or_else(|| AppError {
                    error_type: common::errors::AppErrorTypes::ClientError,
                    message: error_messages::MISSING_ABILITY_REFERENCE.to_string(),
                })?;

            let current_targets = focused_character
                .combatant_properties
                .ability_targets
                .as_ref()
                .ok_or_else(|| AppError {
                    error_type: common::errors::AppErrorTypes::ClientError,
                    message: error_messages::TRIED_TO_CYCLE_TARGETS_WHEN_NO_TARGETS.to_string(),
                })?;

            let new_targets = ability_name.get_next_or_previous_targets(
                current_targets,
                direction,
                &focused_character_id,
                party,
            )?;

            let new_preferences = focused_character
                .combatant_properties
                .ability_target_preferences
                .get_updated_preferences(ability_name, &new_targets, &party);
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
    });
}