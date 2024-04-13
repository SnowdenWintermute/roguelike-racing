use crate::comm_channels::CameraPosition;
use crate::comm_channels::MessageFromBevy;
use crate::comm_channels::YewTransmitter;
use yewdux::Store;

#[derive(Store, PartialEq, Clone)]
pub struct BevyCommunicationStore {
    pub transmitter_option: Option<YewTransmitter>,
    pub messages_from_bevy: Vec<MessageFromBevy>,
    pub bevy_assets_loaded: bool,
    pub camera_position: CameraPosition,
}

impl Default for BevyCommunicationStore {
    fn default() -> Self {
        Self {
            transmitter_option: None,
            messages_from_bevy: Vec::new(),
            bevy_assets_loaded: false,
            camera_position: CameraPosition::default(),
        }
    }
}
