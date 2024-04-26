use self::app::YewApp;
use crate::comm_channels::BevyTransmitter;
use crate::comm_channels::YewTransmitter;
use crate::SharedState;
use std::sync::Arc;
use std::sync::Mutex;
use yew::Properties;
pub mod app;
pub mod components;
pub mod router;
pub mod store;

#[derive(Properties)]
pub struct Props {
    pub shared: Arc<Mutex<SharedState>>,
    pub transmitter: YewTransmitter,
    pub bevy_transmitter: BevyTransmitter,
}

impl PartialEq for Props {
    fn eq(&self, _other: &Self) -> bool {
        true
    }
}

pub fn yew_main(
    yew_transmitter: YewTransmitter,
    bevy_transmitter: BevyTransmitter,
    shared: Arc<Mutex<SharedState>>,
) {
    let document = gloo::utils::document();
    let root = document.query_selector("#yew").unwrap().unwrap();
    let props = Props {
        transmitter: yew_transmitter,
        bevy_transmitter,
        shared,
    };
    yew::Renderer::<YewApp>::with_root_and_props(root, props).render();
}
