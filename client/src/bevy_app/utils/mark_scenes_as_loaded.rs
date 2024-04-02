#![allow(unused)]
use bevy::{prelude::*, scene::SceneInstance};
use gloo::console::info;

use crate::bevy_app::modular_character_plugin::spawn_scenes::{SceneLoaded, SceneName};

pub fn mark_scenes_as_loaded(
    mut commands: Commands,
    scene_manager: Res<SceneSpawner>,
    unloaded_instances: Query<(Entity, &SceneInstance, &SceneName), Without<SceneLoaded>>,
) {
    for (entity, instance, scene_name) in unloaded_instances.iter() {
        if scene_manager.instance_is_ready(**instance) {
            commands.entity(entity).insert(SceneLoaded);
            let name = &scene_name.0;
            info!(format!("marked scene as loaded: {} {:?}", name, entity));
        }
    }
}
