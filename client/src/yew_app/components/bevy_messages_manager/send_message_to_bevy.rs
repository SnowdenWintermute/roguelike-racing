use crate::comm_channels::messages_from_yew::MessageFromYew;
use crate::comm_channels::YewTransmitter;
use common::app_consts::error_messages;
use common::errors::AppError;

pub fn send_message_to_bevy(
    bevy_transmitter_option: &Option<YewTransmitter>,
    message: MessageFromYew,
) -> Result<(), AppError> {
    match bevy_transmitter_option {
        Some(transmitter) => match transmitter.send(message) {
            Ok(_) => Ok(()),
            Err(e) => Err(AppError {
                error_type: common::errors::AppErrorTypes::ClientError,
                message: format!("{:#?}", e),
            }),
        },
        None => Err(AppError {
            error_type: common::errors::AppErrorTypes::ClientError,
            message: error_messages::NO_YEW_TRANSMITTER_TO_BEVY.to_string(),
        }),
    }
}
