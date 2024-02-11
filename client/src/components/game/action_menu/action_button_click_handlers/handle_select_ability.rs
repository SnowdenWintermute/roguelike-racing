use crate::components::alerts::set_alert;
use crate::components::websocket_manager::send_client_input::send_client_input;
use crate::store::alert_store::AlertStore;
use crate::store::game_store::get_cloned_current_battle_option;
use crate::store::game_store::GameStore;
use crate::store::lobby_store::LobbyStore;
use common::app_consts::error_messages;
use common::combatants::abilities::CombatantAbilityNames;
use common::errors::AppError;
use common::game::getters::get_mut_party;
use common::game::getters::get_player;
use common::packets::client_to_server::ClientSelectAbilityPacket;
use common::packets::client_to_server::PlayerInputs;
use gloo::console::log;
use std::rc::Rc;
use web_sys::WebSocket;
use yewdux::prelude::Dispatch;

pub fn handle_select_ability(
    game_dispatch: Dispatch<GameStore>,
    lobby_state: Rc<LobbyStore>,
    alert_dispatch: Dispatch<AlertStore>,
    websocket_option: &Option<WebSocket>,
    ability_name_option: Option<CombatantAbilityNames>,
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
        let character_positions = party.character_positions.clone();
        let battle_id_option = party.battle_id;
        let combat_action_properties_option = if let Some(ability_name) = &ability_name_option {
            Some(ability_name.get_attributes().combat_action_properties)
        } else {
            None
        };
        let focused_character = party
            .characters
            .get(&game_store.focused_character_id)
            .ok_or_else(|| AppError {
                error_type: common::errors::AppErrorTypes::ClientError,
                message: error_messages::CHARACTER_NOT_FOUND.to_string(),
            })?;
        let focused_character_id = focused_character.entity_properties.id;

        let username = &lobby_state.username;
        let player = get_player(game, username.to_string())?;
        let player_character_ids_option = player.character_ids.clone();

        let new_targets_option = game.assign_character_initial_targets_on_combat_action_selection(
            focused_character_id,
            &player_character_ids_option,
            party_id,
            battle_id_option,
            &character_positions,
            &combat_action_properties_option,
        )?;
        log!(format!(
            "assigned targets on ability selection: {:#?}",
            new_targets_option
        ));

        let party = get_mut_party(game, party_id)?;
        let focused_character = party
            .characters
            .get_mut(&game_store.focused_character_id)
            .ok_or_else(|| AppError {
                error_type: common::errors::AppErrorTypes::ClientError,
                message: error_messages::CHARACTER_NOT_FOUND.to_string(),
            })?;

        focused_character.combatant_properties.selected_ability_name = ability_name_option.clone();

        send_client_input(
            websocket_option,
            PlayerInputs::SelectAbility(ClientSelectAbilityPacket {
                character_id: focused_character.entity_properties.id,
                ability_name_option,
            }),
        );

        Ok(())
    });

    if let Some(err) = result.err() {
        set_alert(alert_dispatch, err.message)
    }
}
