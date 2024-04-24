use super::process_combatant_model_actions::ActiveModelActions;
use super::process_combatant_model_actions::ModelActionQueue;
use super::process_combatant_model_actions::TransformManager;
use super::spawn_scenes::spawn_scene;
use super::CombatantsById;
use super::HomeLocation;
use super::SkeletonsAwaitingCombatantAssignment;
use crate::bevy_app::asset_loader_plugin::MyAssets;
use crate::comm_channels::messages_from_bevy::MessageFromBevy;
use crate::comm_channels::BevyTransmitter;
use crate::comm_channels::CharacterSpawnEvent;
use crate::frontend_common::CharacterPartCategories;
use crate::frontend_common::CombatantSpecies;
use bevy::gltf::Gltf;
use bevy::prelude::*;
use bevy_mod_billboard::BillboardTextBundle;
use common::combat::ActionResult;
use common::combatants::CombatantProperties;
use common::primatives::EntityId;
use std::collections::HashMap;
use std::collections::HashSet;

// CHARACTER COMPONENTS
#[derive(Component)]
pub struct CombatantIdComponent(pub u32);
#[derive(Component)]
pub struct CombatantMainArmatureMarker;
#[derive(Component)]
pub struct CombatantMainArmatureEntityLink(pub Entity);
#[derive(Component, Debug)]
pub struct MainSkeletonEntity(pub Entity);
#[derive(Component, Debug)]
pub struct MainSkeletonBonesAndArmature(pub HashMap<String, Entity>, pub Entity);
#[derive(Component, Debug)]
pub struct CombatantPropertiesComponent(pub CombatantProperties);
#[derive(Component, Debug, Default)]
pub struct ActionResultsProcessing(pub Vec<ActionResult>);
/// Queue of part entities waiting for spawn. Using Vec in case multiple part scenes get queued
/// from part change requests before they are spawned
#[derive(Component, Default)]
pub struct CharacterPartScenesAwaitingSpawn(pub HashMap<CharacterPartCategories, HashSet<Entity>>);
#[derive(Component, Default)]
pub struct CharacterAttachedPartScenes(pub HashMap<CharacterPartCategories, Entity>);
#[derive(Component, Default, Clone)]
pub struct HitboxRadius(pub f32);
#[derive(Component, Clone, Debug)]
pub struct CombatantSpeciesComponent(pub CombatantSpecies);

pub fn spawn_combatants(
    mut commands: Commands,
    mut character_spawn_event_reader: EventReader<CharacterSpawnEvent>,
    asset_pack: Res<MyAssets>,
    assets_gltf: Res<Assets<Gltf>>,
    mut characters_by_id: ResMut<CombatantsById>,
    mut skeletons_awaiting_combatant_assignment: ResMut<SkeletonsAwaitingCombatantAssignment>,
    transmitter: ResMut<BevyTransmitter>,
) {
    for event in character_spawn_event_reader.read() {
        let character_id = event.0;
        let home_location = &event.1;
        let species = &event.2;
        let combatant_properties = event.3.clone();

        let file_name = match species {
            CombatantSpecies::Humanoid => "main_skeleton.glb",
            CombatantSpecies::Wasp => "wasp_main_skeleton.glb",
            CombatantSpecies::Frog => "frog_main_skeleton.glb",
        };

        let skeleton_handle = asset_pack
            .main_skeletons_with_animations
            .get(file_name)
            .expect("to have loaded the skeleton glb");

        spawn_combatant(
            &mut commands,
            &asset_pack,
            &assets_gltf,
            &mut characters_by_id,
            &mut skeletons_awaiting_combatant_assignment,
            home_location.clone(),
            character_id,
            &transmitter,
            skeleton_handle,
            file_name.to_string(),
            species.clone(),
            combatant_properties,
        )
    }
}

pub fn spawn_combatant(
    commands: &mut Commands,
    asset_pack: &Res<MyAssets>,
    assets_gltf: &Res<Assets<Gltf>>,
    characters_by_id: &mut ResMut<CombatantsById>,
    skeletons_awaiting_combatant_assignment: &mut ResMut<SkeletonsAwaitingCombatantAssignment>,
    home_location: HomeLocation,
    character_id: EntityId,
    transmitter: &ResMut<BevyTransmitter>,
    skeleton_handle: &Handle<Gltf>,
    file_name: String,
    species: CombatantSpecies,
    combatant_properties: CombatantProperties,
) {
    // - spawn skeleton and store its entity id on the character

    let skeleton_entity = spawn_scene(
        commands,
        &assets_gltf,
        skeleton_handle.clone(),
        file_name,
        false,
        home_location.clone(),
    )
    .expect("to have a skeleton gltf handle");

    // - add skeleton entity to skeletons_awaiting_combatant_assignment resource
    skeletons_awaiting_combatant_assignment
        .0
        .insert(character_id, (skeleton_entity, species.clone()));

    let transform_manager = TransformManager::new(home_location.clone());

    let character_entity_commands = commands.spawn((
        CombatantIdComponent(character_id),
        MainSkeletonEntity(skeleton_entity),
        CharacterAttachedPartScenes(HashMap::new()),
        CharacterPartScenesAwaitingSpawn(HashMap::new()),
        home_location,
        transform_manager,
        ModelActionQueue::default(),
        ActiveModelActions::default(),
        HitboxRadius(0.7),
        CombatantPropertiesComponent(combatant_properties),
        ActionResultsProcessing::default(),
    ));

    let character_entity = character_entity_commands.id();
    // - add character id to list of characters resource
    characters_by_id.0.insert(character_id, character_entity);

    // BILLBOARD
    let font_handle = asset_pack
        .font_files
        .get("FiraSans-Regular.ttf")
        .expect("to have loaded the font");

    let mut billboard_entity_commands = commands.spawn(BillboardTextBundle {
        transform: Transform::from_xyz(0.0, 2.0, 0.0).with_scale(Vec3::splat(0.003)),
        text: Text::from_sections([TextSection {
            value: format!("Character {}", character_id),
            style: TextStyle {
                font_size: 60.0,
                font: font_handle.clone(),
                color: Color::WHITE,
            },
        }]),
        ..Default::default()
    });

    billboard_entity_commands.set_parent(skeleton_entity);

    // NOTIFY YEW
    let _ = transmitter.send(MessageFromBevy::CombatantSpawned(character_id));
}
