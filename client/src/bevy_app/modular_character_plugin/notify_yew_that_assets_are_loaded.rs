use crate::comm_channels::{BevyTransmitter, MessageFromBevy};
use bevy::prelude::*;

pub fn notify_yew_that_assets_are_loaded(transmitter: ResMut<BevyTransmitter>) {
    transmitter.0.send(MessageFromBevy::AssetsLoaded);
}
