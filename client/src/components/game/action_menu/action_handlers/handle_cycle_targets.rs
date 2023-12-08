use crate::{
    components::websocket_manager::send_client_input::send_client_input,
    store::game_store::GameStore,
};
use common::{
    app_consts::error_messages,
    combatants::abilities::AbilityTarget,
    errors::AppError,
    game::getters::get_mut_party,
    packets::client_to_server::{ChangeTargetsPacket, PlayerInputs},
    primatives::NextOrPrevious,
};
use web_sys::WebSocket;
use yewdux::prelude::Dispatch;

pub fn handle_cycle_targets(
    game_dispatch: Dispatch<GameStore>,
    websocket_option: &Option<WebSocket>,
    direction: &NextOrPrevious,
) {
    game_dispatch.reduce_mut(|game_store| {
        let result = move || -> Result<(), AppError> {
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
            let ability_name = &focused_character
                .combatant_properties
                .selected_ability_name
                .ok_or_else(|| AppError {
                    error_type: common::errors::AppErrorTypes::ClientError,
                    message: error_messages::MISSING_ABILITY_REFERENCE.to_string(),
                })?;

            let ability_attributes = ability_name.get_attributes();
            let current_targets = ability_name.targets_by_saved_preference_or_default(
                focused_character_id,
                &focused_character
                    .combatant_properties
                    .ability_target_preferences,
                party,
            )?;

            let new_targets = ability_name.get_next_or_previous_targets(
                current_targets,
                direction,
                character_id,
                party,
            )?;

            send_client_input(
                &websocket_option,
                PlayerInputs::ChangeTargetIds(ChangeTargetsPacket {
                    character_id: focused_character.entity_properties.id,
                    new_targets,
                }),
            );
            Ok(())
        };
    });
}
