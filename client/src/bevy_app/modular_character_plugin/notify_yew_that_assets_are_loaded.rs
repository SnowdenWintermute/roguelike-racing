use crate::comm_channels::BevyTransmitter;
use crate::comm_channels::MessageFromBevy;
use bevy::prelude::*;

pub fn notify_yew_that_assets_are_loaded(transmitter: ResMut<BevyTransmitter>) {
    let _result = transmitter.0.send(MessageFromBevy::AssetsLoaded);
}
