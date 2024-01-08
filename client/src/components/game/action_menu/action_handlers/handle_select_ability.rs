use crate::components::websocket_manager::send_client_input::send_client_input;
use crate::store::game_store::get_cloned_current_battle_option;
use crate::store::game_store::GameStore;
use common::app_consts::error_messages;
use common::combatants::abilities::CombatantAbilityNames;
use common::errors::AppError;
use common::game::getters::get_ally_ids_and_opponent_ids_option;
use common::game::getters::get_mut_party;
use common::packets::client_to_server::ClientSelectAbilityPacket;
use common::packets::client_to_server::PlayerInputs;
use gloo::console::log;
use web_sys::WebSocket;
use yewdux::prelude::Dispatch;

pub fn handle_select_ability(
    game_dispatch: Dispatch<GameStore>,
    websocket_option: &Option<WebSocket>,
    ability_name: CombatantAbilityNames,
) {
    let result = game_dispatch.reduce_mut(|game_store| -> Result<(), AppError> {
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
        let focused_character = party
            .characters
            .get(&game_store.focused_character_id)
            .ok_or_else(|| AppError {
                error_type: common::errors::AppErrorTypes::ClientError,
                message: error_messages::CHARACTER_NOT_FOUND.to_string(),
            })?;

        let target_preferences = &focused_character
            .combatant_properties
            .ability_target_preferences;

        let (ally_ids, opponent_ids_option) = get_ally_ids_and_opponent_ids_option(
            &party.character_positions,
            battle_option.as_ref(),
            focused_character.entity_properties.id,
        )?;

        let targets = ability_name.targets_by_saved_preference_or_default(
            focused_character.entity_properties.id,
            &target_preferences,
            ally_ids.clone(),
            opponent_ids_option.clone(),
        )?;

        let new_target_preferences = target_preferences.get_updated_preferences(
            &ability_name,
            &targets,
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
        focused_character.combatant_properties.selected_ability_name = Some(ability_name.clone());
        focused_character.combatant_properties.ability_targets = Some(targets.clone());
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
    });

    if result.is_err() {
        log!("error selecting ability")
    }
}
