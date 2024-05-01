mod asset_loader_plugin;
pub mod bevy_app_consts;
mod camera_plugin;
pub mod modular_character_plugin;
mod plane_plugin;
pub mod utils;
use self::asset_loader_plugin::AssetLoaderPlugin;
use self::camera_plugin::CameraPlugin;
use self::modular_character_plugin::ModularCharacterPlugin;
use self::plane_plugin::PlanePlugin;
use bevy::asset::AssetMetaCheck;
use bevy::prelude::*;
use bevy::render::Render;
use bevy::render::RenderSet;
use bevy::winit::UpdateMode;
use bevy::winit::WinitSettings;
use bevy_mod_billboard::plugin::BillboardPlugin;
use bevy_panorbit_camera::PanOrbitCameraPlugin;

#[derive(States, Clone, Eq, PartialEq, Default, Hash, Debug)]
pub enum BevyAppState {
    #[default]
    PausedAndHidden,
    Running,
}

pub fn bevy_main(comm_channel_plugin: impl Plugin) {
    App::new()
        .insert_resource(WinitSettings {
            focused_mode: UpdateMode::Continuous,
            unfocused_mode: UpdateMode::Continuous,
        })
        .insert_resource(AssetMetaCheck::Never)
        .insert_resource(ClearColor(Color::rgb(0.1, 0.1, 0.15)))
        .insert_resource(AmbientLight {
            color: Color::default(),
            brightness: 500.0,
        })
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                canvas: Some("#bevy".to_string()),
                prevent_default_event_handling: false,
                ..Default::default()
            }),
            ..default()
        }))
        .init_state::<BevyAppState>()
        .configure_sets(
            Render,
            RenderSet::Render.run_if(in_state(BevyAppState::Running)),
        )
        .add_plugins(comm_channel_plugin)
        .add_plugins(PlanePlugin)
        .add_plugins(CameraPlugin)
        .add_plugins(AssetLoaderPlugin)
        .add_plugins(ModularCharacterPlugin)
        // EXTERNAL
        .add_plugins(PanOrbitCameraPlugin)
        .add_plugins(BillboardPlugin)
        .run();
}
