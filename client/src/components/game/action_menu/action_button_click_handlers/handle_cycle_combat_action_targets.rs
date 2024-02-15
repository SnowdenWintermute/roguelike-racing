use crate::components::websocket_manager::send_client_input::send_client_input;
use crate::store::game_store::GameStore;
use crate::store::lobby_store::LobbyStore;
use common::app_consts::error_messages;
use common::errors::AppError;
use common::game::getters::get_player;
use common::packets::client_to_server::PlayerInputs;
use common::packets::CharacterAndDirection;
use common::primatives::NextOrPrevious;
use std::rc::Rc;
use web_sys::WebSocket;
use yewdux::Dispatch;

pub fn handle_cycle_combat_action_targets(
    game_dispatch: Dispatch<GameStore>,
    websocket_option: &Option<WebSocket>,
    lobby_state: Rc<LobbyStore>,
    direction: &NextOrPrevious,
) -> Result<(), AppError> {
    game_dispatch.reduce_mut(|game_store| -> Result<(), AppError> {
        let game = game_store.game.as_mut().ok_or_else(|| AppError {
            error_type: common::errors::AppErrorTypes::ClientError,
            message: error_messages::MISSING_GAME_REFERENCE.to_string(),
        })?;
        let party_id = game_store.current_party_id.ok_or_else(|| AppError {
            error_type: common::errors::AppErrorTypes::ClientError,
            message: error_messages::MISSING_PARTY_REFERENCE.to_string(),
        })?;

        let character_id = game_store.focused_character_id;
        // CLIENT PREDICTION
        let username = &lobby_state.username;
        let player = get_player(game, &username)?;
        let player_character_ids_option = player.character_ids.clone();
        game.cycle_character_targets(
            party_id,
            player_character_ids_option,
            &username,
            character_id,
            &direction,
        )?;

        send_client_input(
            &websocket_option,
            PlayerInputs::CycleCombatActionTargets(CharacterAndDirection {
                character_id,
                direction: direction.clone(),
            }),
        );

        Ok(())
    })
}
