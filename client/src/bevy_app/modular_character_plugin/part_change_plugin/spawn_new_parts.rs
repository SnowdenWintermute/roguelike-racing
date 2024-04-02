use crate::bevy_app::asset_loader_plugin::MyAssets;
use crate::bevy_app::modular_character_plugin::spawn_combatant::CharacterPartScenesAwaitingSpawn;
use crate::bevy_app::modular_character_plugin::spawn_combatant::CombatantIdComponent;
use crate::bevy_app::modular_character_plugin::spawn_combatant::MainSkeletonBonesAndArmature;
use crate::bevy_app::modular_character_plugin::spawn_scenes::spawn_scene;
use crate::bevy_app::modular_character_plugin::CombatantsById;
use crate::bevy_app::modular_character_plugin::HomeLocation;
use crate::comm_channels::CharacterPartSelectionEvent;
use crate::frontend_common::CharacterPartCategories;
use bevy::gltf::Gltf;
use bevy::prelude::*;

pub fn spawn_new_parts(
    mut commands: Commands,
    mut characters_with_spawned_skeletons: Query<(
        Entity,
        &CombatantIdComponent,
        &mut CharacterPartScenesAwaitingSpawn,
        &MainSkeletonBonesAndArmature,
    )>,
    mut part_selection_event_reader: EventReader<CharacterPartSelectionEvent>,
    character_by_id: Res<CombatantsById>,
    asset_pack: Res<MyAssets>,
    assets_gltf: Res<Assets<Gltf>>,
) {
    for event in part_selection_event_reader.read() {
        info!("part spawner read part selection event: {:#?}", event);
        let file_name = &event.0.name;
        let category = &event.0.category;

        let character_id = event.0.character_id;
        //  - get associated character
        let character_entity = character_by_id
            .0
            .get(&event.0.character_id)
            .expect("to have a character by this id");
        info!(
            "spawning part for character id: {}, entity: {:?}",
            character_id, character_entity
        );
        // ensure it has an assigned skeleton

        if let Ok((_, _, mut parts_awaiting_spawn, _)) =
            characters_with_spawned_skeletons.get_mut(*character_entity)
        {
            spawn_part(
                &file_name,
                &category,
                &mut commands,
                &asset_pack,
                &assets_gltf,
                &mut parts_awaiting_spawn,
            )
        }
    }
}

pub fn spawn_part(
    file_name: &String,
    category: &CharacterPartCategories,
    commands: &mut Commands,
    asset_pack: &Res<MyAssets>,
    assets_gltf: &Res<Assets<Gltf>>,
    parts_awaiting_spawn: &mut CharacterPartScenesAwaitingSpawn,
) {
    let gltf_handle_option = match category {
        CharacterPartCategories::Head => asset_pack.heads.get(file_name),
        CharacterPartCategories::Torso => asset_pack.torsos.get(file_name),
        CharacterPartCategories::Leg => asset_pack.legs.get(file_name),
        CharacterPartCategories::Weapon => asset_pack.weapons.get(file_name),
        CharacterPartCategories::FullBodyMesh => asset_pack.non_humanoids.get(file_name),
    };
    //  - spawn new part and store entity id and category on character "awaiting spawn"
    let gltf_handle = gltf_handle_option.expect("to have loaded the gltf file asset");
    let part_scene_entity = spawn_scene(
        commands,
        assets_gltf,
        gltf_handle.clone(),
        file_name.clone(),
        true,
        HomeLocation::default(),
    )
    .expect("to spawn the scene");
    // info!("spawned part scene: {:?}", part_scene_entity);

    parts_awaiting_spawn
        .0
        .entry(category.clone())
        .or_default()
        .insert(part_scene_entity);
}
