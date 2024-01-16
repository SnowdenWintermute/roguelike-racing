use crate::components::mesh_manager::CombatantAnimation;
use crate::store::game_store::GameStore;
use common::app_consts::error_messages;
use common::errors::AppError;
use yewdux::Dispatch;

pub fn queue_return_to_home_position_animations(
    game_dispatch: Dispatch<GameStore>,
    combatant_id: u32,
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

        event_manager
            .animation_queue
            .push_back(CombatantAnimation::ReturnToReadyPosition);

        Ok(())
    })
}
