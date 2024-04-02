use crate::yew_app::store::game_store::GameStore;
use common::app_consts::error_messages;
use common::errors::AppError;
use common::packets::server_to_client::ActionResultsPacket;
use gloo::console::log;
use yewdux::Dispatch;

pub fn handle_raw_action_results(
    game_dispatch: Dispatch<GameStore>,
    packet: ActionResultsPacket,
) -> Result<(), AppError> {
    game_dispatch.reduce_mut(|store| {
        log!("got raw action results");
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
    })
}
