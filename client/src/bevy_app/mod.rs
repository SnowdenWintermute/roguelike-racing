mod asset_loader_plugin;
mod camera_plugin;
pub mod modular_character_plugin;
mod plane_plugin;
pub mod utils;
use self::asset_loader_plugin::AssetLoaderPlugin;
use self::camera_plugin::CameraPlugin;
use self::modular_character_plugin::ModularCharacterPlugin;
use self::plane_plugin::PlanePlugin;
use crate::SharedState;
use bevy::asset::AssetMetaCheck;
use bevy::prelude::*;
use bevy::winit::UpdateMode;
use bevy::winit::WinitSettings;
use bevy_mod_billboard::plugin::BillboardPlugin;
use bevy_panorbit_camera::PanOrbitCameraPlugin;
use std::sync::Arc;
use std::sync::Mutex;

#[derive(Resource)]
pub struct SharedResource(Arc<Mutex<SharedState>>);

pub fn bevy_main(comm_channel_plugin: impl Plugin, shared_state: Arc<Mutex<SharedState>>) {
    App::new()
        .insert_resource(WinitSettings {
            focused_mode: UpdateMode::Continuous,
            unfocused_mode: UpdateMode::Continuous,
        })
        .insert_resource(AssetMetaCheck::Never)
        .insert_resource(ClearColor(Color::rgb(0.1, 0.1, 0.15)))
        .insert_resource(AmbientLight {
            color: Color::default(),
            brightness: 1000.0,
        })
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                canvas: Some("#bevy".to_string()),
                ..Default::default()
            }),
            ..default()
        }))
        .add_plugins(comm_channel_plugin)
        .insert_resource(SharedResource(shared_state))
        .add_plugins(PlanePlugin)
        .add_plugins(CameraPlugin)
        .add_plugins(AssetLoaderPlugin)
        .add_plugins(ModularCharacterPlugin)
        // EXTERNAL
        .add_plugins(PanOrbitCameraPlugin)
        .add_plugins(BillboardPlugin)
        .run();
}
