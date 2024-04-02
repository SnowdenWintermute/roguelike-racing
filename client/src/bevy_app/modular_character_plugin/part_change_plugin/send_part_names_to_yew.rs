use crate::{
    bevy_app::asset_loader_plugin::MyAssets,
    comm_channels::{BevyTransmitter, MessageFromBevy},
    frontend_common::PartsByName,
};
use bevy::prelude::*;

pub fn send_part_names_to_yew(asset_pack: Res<MyAssets>, transmitter: ResMut<BevyTransmitter>) {
    let mut part_names = PartsByName::default();

    for (name, _) in &asset_pack.heads {
        part_names.heads.insert(name.clone());
    }
    for (name, _) in &asset_pack.torsos {
        part_names.torsos.insert(name.clone());
    }
    for (name, _) in &asset_pack.legs {
        part_names.legs.insert(name.clone());
    }
    for (name, _) in &asset_pack.weapons {
        part_names.weapons.insert(name.clone());
    }

    // info!("sent part names :{:?}", part_names);
    let _ = transmitter.send(MessageFromBevy::PartNames(part_names));
}
