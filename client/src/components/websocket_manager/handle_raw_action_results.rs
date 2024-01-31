use crate::store::game_store::GameStore;
use common::app_consts::error_messages;
use common::errors::AppError;
use common::packets::server_to_client::ActionResultsPacket;

pub fn handle_raw_action_results(
    store: &mut GameStore,
    packet: ActionResultsPacket,
) -> Result<(), AppError> {
    for action_result in packet.action_results {
        store
            .action_results_manager
            .combantant_event_managers
            .get_mut(&packet.action_taker_id)
            .ok_or_else(|| AppError {
                error_type: common::errors::AppErrorTypes::ClientError,
                message: error_messages::COMBANTANT_EVENT_MANAGER_NOT_FOUND.to_string(),
            })?
            .action_result_queue
            .push_back(action_result);
    }
    Ok(())
}
