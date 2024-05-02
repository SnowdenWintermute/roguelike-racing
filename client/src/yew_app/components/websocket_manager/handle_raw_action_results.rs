use crate::comm_channels::messages_from_yew::MessageFromYew;
use crate::yew_app::store::bevy_communication_store::BevyCommunicationStore;
use common::app_consts::error_messages;
use common::errors::AppError;
use common::packets::server_to_client::ActionResultsPacket;
use yewdux::Dispatch;

pub fn handle_raw_action_results(
    bevy_communication_dispatch: Dispatch<BevyCommunicationStore>,
    packet: ActionResultsPacket,
) -> Result<(), AppError> {
    bevy_communication_dispatch.reduce_mut(|store| {
        let _result = store
            .transmitter_option
            .as_ref()
            .ok_or_else(|| AppError {
                error_type: common::errors::AppErrorTypes::ClientError,
                message: error_messages::NO_YEW_TRANSMITTER_TO_BEVY.to_string(),
            })?
            .send(MessageFromYew::NewRawActionResults(
                packet.action_taker_id,
                packet.action_results,
            ));
        Ok(())
    })
}
