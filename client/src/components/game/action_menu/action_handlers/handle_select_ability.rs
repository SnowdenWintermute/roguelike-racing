use crate::{
    components::websocket_manager::send_client_input::send_client_input,
    store::game_store::GameStore,
};
use common::{
    app_consts::error_messages,
    combatants::abilities::{
        get_combatant_ability_attributes::TargetingScheme, CombatantAbilityNames,
    },
    errors::AppError,
    game::getters::get_mut_party,
    packets::client_to_server::{ClientSelectAbilityPacket, PlayerInputs},
};
use web_sys::WebSocket;
use yewdux::prelude::Dispatch;

pub fn handle_select_ability(
    game_dispatch: Dispatch<GameStore>,
    websocket_option: &Option<WebSocket>,
    ability_name: CombatantAbilityNames,
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
            let ability = focused_character
                .combatant_properties
                .abilities
                .get(&ability_name)
                .ok_or_else(|| AppError {
                    error_type: common::errors::AppErrorTypes::ClientError,
                    message: error_messages::ABILITY_NOT_OWNED.to_string(),
                })?;

            let target_preferences = &focused_character
                .combatant_properties
                .ability_target_preferences;

            let targets = ability_name.targets_by_saved_preference_or_default(
                focused_character.entity_properties.id,
                &target_preferences,
                party,
            )?;

            let new_target_preferences =
                target_preferences.get_updated_preferences(&ability_name, &targets, party);

            let focused_character = party
                .characters
                .get_mut(&game_store.focused_character_id)
                .ok_or_else(|| AppError {
                    error_type: common::errors::AppErrorTypes::ClientError,
                    message: error_messages::CHARACTER_NOT_FOUND.to_string(),
                })?;
            focused_character.combatant_properties.selected_ability_name =
                Some(ability_name.clone());
            focused_character.combatant_properties.ability_target_ids = new_target_ids.clone();
            focused_character
                .combatant_properties
                .ability_target_preferences = new_target_preferences;

            send_client_input(
                websocket_option,
                PlayerInputs::SelectAbility(ClientSelectAbilityPacket {
                    character_id: focused_character.entity_properties.id,
                    ability_name_option: Some(ability_name),
                }),
            );

            Ok(())
        };
    });
}
