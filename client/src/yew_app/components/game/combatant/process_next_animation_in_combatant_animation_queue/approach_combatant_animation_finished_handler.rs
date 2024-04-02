use crate::yew_app::components::mesh_manager::CombatantVisualLocation;
use crate::yew_app::store::game_store::GameStore;
use common::app_consts::error_messages;
use common::errors::AppError;
use yewdux::Dispatch;

pub fn approach_combatant_animation_finished_handler(
    game_dispatch: Dispatch<GameStore>,
    combatant_id: u32,
    target_id: u32,
) -> Result<(), AppError> {
    game_dispatch.reduce_mut(|store| -> Result<(), AppError> {
        let event_manager = store
            .action_results_manager
            .combantant_event_managers
            .get_mut(&combatant_id)
            .ok_or_else(|| AppError {
                error_type: common::errors::AppErrorTypes::ClientError,
                message: error_messages::COMBANTANT_EVENT_MANAGER_NOT_FOUND.to_string(),
            })?;
        event_manager.visual_location = CombatantVisualLocation::StandingInFrontOf(target_id);
        Ok(())
    })
}
