use super::handle_cycle_combat_action_targets::handle_cycle_combat_action_targets;
use crate::components::websocket_manager::send_client_input::send_client_input;
use crate::store::game_store::GameStore;
use common::app_consts::error_messages;
use common::errors::AppError;
use common::game::getters::get_mut_party;
use common::packets::client_to_server::ChangeTargetsPacket;
use common::packets::client_to_server::PlayerInputs;
use common::primatives::NextOrPrevious;
use gloo::console::log;
use web_sys::WebSocket;
use yewdux::prelude::Dispatch;

pub fn handle_cycle_ability_targets(
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
            let ability_name = ability_name.clone();
            let ability_attributes = ability_name.get_attributes();
            let combat_action_properties = ability_attributes.combat_action_properties;
            let new_targets = handle_cycle_combat_action_targets(
                game_store,
                combat_action_properties,
                direction,
            )?;

            send_client_input(
                &websocket_option,
                PlayerInputs::ChangeAbilityTargets(ChangeTargetsPacket {
                    character_id: focused_character_id,
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
            log!(format!("{:#?}", result.err()))
        }
    });
}
