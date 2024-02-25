use crate::components::alerts::set_alert;
use crate::components::websocket_manager::send_client_input::send_client_input;
use crate::store::alert_store::AlertStore;
use crate::store::game_store::GameStore;
use crate::store::lobby_store::LobbyStore;
use common::app_consts::error_messages;
use common::combat::combat_actions::CombatAction;
use common::errors::AppError;
use common::game::getters::get_mut_party;
use common::game::getters::get_player;
use common::packets::client_to_server::CharacterAndCombatAction;
use common::packets::client_to_server::PlayerInputs;
use gloo::console::log;
use std::rc::Rc;
use web_sys::WebSocket;
use yewdux::prelude::Dispatch;

pub fn handle_select_combat_action(
    game_dispatch: Dispatch<GameStore>,
    alert_dispatch: Dispatch<AlertStore>,
    lobby_state: Rc<LobbyStore>,
    websocket_option: &Option<WebSocket>,
    combat_action_option: Option<CombatAction>,
) {
    let result = game_dispatch.reduce_mut(|game_store| -> Result<(), AppError> {
        game_store.hovered_action = None;
        let game = game_store.game.as_mut().ok_or_else(|| AppError {
            error_type: common::errors::AppErrorTypes::ClientError,
            message: error_messages::MISSING_GAME_REFERENCE.to_string(),
        })?;
        let party_id = game_store.current_party_id.ok_or_else(|| AppError {
            error_type: common::errors::AppErrorTypes::ClientError,
            message: error_messages::MISSING_PARTY_REFERENCE.to_string(),
        })?;
        let party = get_mut_party(game, party_id)?;
        let battle_id_option = party.battle_id;
        let character_positions = party.character_positions.clone();
        let character = party
            .characters
            .get(&game_store.focused_character_id)
            .ok_or_else(|| AppError {
                error_type: common::errors::AppErrorTypes::ClientError,
                message: error_messages::CHARACTER_NOT_FOUND.to_string(),
            })?;
        let character_id = character.entity_properties.id;

        let combat_action_properties_option = match &combat_action_option {
            Some(combat_action) => Some(combat_action.get_properties_if_owned(game, character_id)?),
            None => None,
        };

        let username = &lobby_state.username;
        log!(format!("selecting ability - player username :{username}"));
        let player = get_player(game, &username)?;
        let player_character_ids_option = player.character_ids.clone();

        let _ = game.assign_character_action_targets(
            character_id,
            &player_character_ids_option,
            &username,
            party_id,
            battle_id_option,
            &character_positions,
            &combat_action_properties_option,
        )?;

        let party = get_mut_party(game, party_id)?;
        let character =
            party.get_mut_character_if_owned(player_character_ids_option.clone(), character_id)?;
        character.combatant_properties.selected_combat_action = combat_action_option.clone();

        send_client_input(
            websocket_option,
            PlayerInputs::SelectCombatAction(CharacterAndCombatAction {
                character_id: character.entity_properties.id,
                combat_action_option,
            }),
        );

        Ok(())
    });

    if let Some(err) = result.err() {
        set_alert(alert_dispatch, err.message)
    }
}
