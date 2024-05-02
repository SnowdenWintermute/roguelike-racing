use crate::comm_channels::messages_from_yew::MessageFromYew;
use crate::yew_app::store::bevy_communication_store::BevyCommunicationStore;
use common::app_consts::error_messages;
use common::combat::CombatTurnResult;
use common::errors::AppError;
use yewdux::Dispatch;

pub fn handle_combat_turn_results(
    bevy_communication_dispatch: Dispatch<BevyCommunicationStore>,
    turn_results: Vec<CombatTurnResult>,
) -> Result<(), AppError> {
    bevy_communication_dispatch.reduce_mut(|store| {
        let _result = store
            .transmitter_option
            .as_ref()
            .ok_or_else(|| AppError {
                error_type: common::errors::AppErrorTypes::ClientError,
                message: error_messages::NO_YEW_TRANSMITTER_TO_BEVY.to_string(),
            })?
            .send(MessageFromYew::NewTurnResults(turn_results.into()));
        Ok(())
    })
}
