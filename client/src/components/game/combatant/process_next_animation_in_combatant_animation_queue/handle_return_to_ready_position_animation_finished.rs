use crate::components::websocket_manager::handle_combat_turn_results::send_next_turn_result_to_combatant_event_manager;
use crate::store::game_store::GameStore;
use common::app_consts::error_messages;
use common::errors::AppError;
use yewdux::Dispatch;

pub fn handle_return_to_ready_position_animation_finished(
    game_dispatch: Dispatch<GameStore>,
) -> Result<(), AppError> {
    // if in battle, call for next turn result to be passed to it's enitity
    let battle_id_option =
        game_dispatch.reduce_mut(|store| -> Option<u32> { store.current_battle_id });
    if let Some(battle_id) = battle_id_option {
        game_dispatch.reduce_mut(|store| -> Result<(), AppError> {
            let game = store.game.as_mut().ok_or_else(|| AppError {
                error_type: common::errors::AppErrorTypes::ClientError,
                message: error_messages::GAME_NOT_FOUND.to_string(),
            })?;
            game.end_active_combatant_turn(battle_id)?;
            Ok(())
        })?;
        send_next_turn_result_to_combatant_event_manager(game_dispatch)
    } else {
        Ok(())
    }
}
