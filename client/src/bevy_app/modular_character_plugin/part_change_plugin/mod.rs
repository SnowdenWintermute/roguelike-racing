mod attach_newly_loaded_part_scenes;
mod despawn_attached_part;
pub mod send_part_names_to_yew;
pub mod spawn_new_parts;
use self::{
    attach_newly_loaded_part_scenes::attach_newly_loaded_part_scenes,
    send_part_names_to_yew::send_part_names_to_yew, spawn_new_parts::spawn_new_parts,
};
use crate::bevy_app::asset_loader_plugin::AssetLoaderState;
use bevy::prelude::*;

pub struct PartChangePlugin;
impl Plugin for PartChangePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AssetLoaderState::Done), send_part_names_to_yew)
            .add_systems(
                Update,
                (
                    (spawn_new_parts, attach_newly_loaded_part_scenes).chain()
                    // mark_scenes_as_loaded,
                )
                .run_if(in_state(AssetLoaderState::Done)),
            );
    }
}
