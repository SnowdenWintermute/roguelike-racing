use crate::components::mesh_manager::CombatantVisualLocation;
use crate::components::websocket_manager::handle_combat_turn_results::send_next_turn_result_to_combatant_event_manager;
use crate::store::game_store::GameStore;
use common::app_consts::error_messages;
use common::errors::AppError;
use yewdux::Dispatch;

pub fn return_to_ready_position_animation_finished_handler(
    game_dispatch: Dispatch<GameStore>,
    combatant_id: u32,
) -> Result<(), AppError> {
    // if in battle, call for next turn result to be passed to it's enitity
    let cloned_game_dispatch = game_dispatch.clone();
    game_dispatch.reduce_mut(|store| -> Result<(), AppError> {
        let event_manager = store
            .action_results_manager
            .combantant_event_managers
            .get_mut(&combatant_id)
            .ok_or_else(|| AppError {
                error_type: common::errors::AppErrorTypes::ClientError,
                message: error_messages::COMBANTANT_EVENT_MANAGER_NOT_FOUND.to_string(),
            })?;
        event_manager.visual_location = CombatantVisualLocation::HomePosition;

        let battle_id_option = store.current_battle_id;
        if let Some(battle_id) = battle_id_option {
            let game = store.game.as_mut().ok_or_else(|| AppError {
                error_type: common::errors::AppErrorTypes::ClientError,
                message: error_messages::GAME_NOT_FOUND.to_string(),
            })?;
            game.end_active_combatant_turn(battle_id)?;
            send_next_turn_result_to_combatant_event_manager(cloned_game_dispatch)
        } else {
            Ok(())
        }
    })
}
