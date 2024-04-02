use super::Animations;
use crate::bevy_app::asset_loader_plugin::AssetLoaderState;
use crate::bevy_app::asset_loader_plugin::MyAssets;
use crate::comm_channels::BevyTransmitter;
use crate::comm_channels::MessageFromBevy;
use bevy::gltf::Gltf;
use bevy::prelude::*;
use std::collections::HashSet;

pub fn register_animations(
    asset_pack: Res<MyAssets>,
    assets_gltf: Res<Assets<Gltf>>,
    mut animations: ResMut<Animations>,
    mut next_state: ResMut<NextState<AssetLoaderState>>,
    transmitter: ResMut<BevyTransmitter>,
) {
    let mut skeleton_gltfs = Vec::new();

    for skeleton_handle in asset_pack.main_skeletons_with_animations.iter() {
        skeleton_gltfs.push(
            assets_gltf
                .get(skeleton_handle.1)
                .expect("to have loaded the main_skeleton.glb"),
        );
    }

    let mut animation_names_for_yew: HashSet<String> = HashSet::new();

    info!("inserting animations");
    for gltf in skeleton_gltfs {
        for named_animation in gltf.named_animations.iter() {
            // info!("inserting animations: {}", named_animation.0);
            animations.0.insert(
                named_animation.0.clone(),
                gltf.named_animations[named_animation.0].clone(),
            );
            animation_names_for_yew.insert(named_animation.0.clone());
        }
    }
    let _ = transmitter.0.send(MessageFromBevy::AnimationsAvailable(
        animation_names_for_yew,
    ));

    next_state.set(AssetLoaderState::Done)
}
