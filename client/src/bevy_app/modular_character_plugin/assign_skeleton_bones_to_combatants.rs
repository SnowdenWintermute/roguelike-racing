use std::f32::consts::PI;

use super::assemble_parts::get_main_skeleton_bones_and_armature::get_main_skeleton_bones_and_armature;
use super::part_change_plugin::spawn_new_parts::spawn_part;
use super::spawn_combatant::CharacterPartScenesAwaitingSpawn;
use super::spawn_combatant::CombatantMainArmatureEntityLink;
use super::spawn_combatant::CombatantMainArmatureMarker;
use super::spawn_combatant::CombatantSpeciesComponent;
use super::spawn_combatant::MainSkeletonBonesAndArmature;
use super::spawn_scenes::SceneLoaded;
use super::CombatantsById;
use super::SkeletonsAwaitingCombatantAssignment;
use crate::bevy_app::asset_loader_plugin::MyAssets;
use crate::frontend_common::CharacterPartCategories;
use crate::frontend_common::CombatantSpecies;
use bevy::gltf::Gltf;
use bevy::prelude::*;
use bevy::scene::SceneInstance;

pub fn assign_skeleton_bones_to_combatants(
    mut commands: Commands,
    scene_manager: Res<SceneSpawner>,
    unloaded_instances: Query<(Entity, &SceneInstance), Without<SceneLoaded>>,
    mut skeletons_awaiting_combatant_assignment: ResMut<SkeletonsAwaitingCombatantAssignment>,
    mut parts_awaiting_spawn_query: Query<&mut CharacterPartScenesAwaitingSpawn>,
    characters_by_id: Res<CombatantsById>,
    all_entities_with_children: Query<&Children>,
    names: Query<&Name>,
    asset_pack: Res<MyAssets>,
    assets_gltf: Res<Assets<Gltf>>,
) {
    //   - loop unspawned skeletons and check for readiness
    let mut character_ids_of_skeletons_readied = Vec::new();
    for (character_id, (skeleton_entity, species)) in
        skeletons_awaiting_combatant_assignment.0.iter()
    {
        if let Ok((entity, scene_instance)) = unloaded_instances.get(*skeleton_entity) {
            if !scene_manager.instance_is_ready(**scene_instance) {
                continue;
            }
            // mark as loaded
            commands.entity(entity).insert(SceneLoaded);
            // remove skeleton entity from skeletons_awaiting_combatant_assignment resource
            character_ids_of_skeletons_readied.push(*character_id);
            // for any spawned, add its bones to the corresponding character
            let (main_skeleton_bones, main_armature_entity) = get_main_skeleton_bones_and_armature(
                &skeleton_entity,
                &all_entities_with_children,
                &names,
            );

            // mark armature to later watch for changes in its children to calc scene AABB
            commands
                .entity(main_armature_entity)
                .insert(CombatantMainArmatureMarker);

            commands
                .entity(*skeleton_entity)
                .insert(CombatantSpeciesComponent(species.clone()));

            let character_entity = characters_by_id
                .0
                .get(character_id)
                .expect("for this character to exist");
            let mut character_entity_commands = commands.entity(*character_entity);
            character_entity_commands.insert(CombatantMainArmatureEntityLink(main_armature_entity));
            character_entity_commands.insert(MainSkeletonBonesAndArmature(
                main_skeleton_bones,
                main_armature_entity,
            ));

            if let Ok(mut parts_awaiting_spawn) =
                parts_awaiting_spawn_query.get_mut(*character_entity)
            {
                let parts_to_spawn = match species {
                    CombatantSpecies::Humanoid => Vec::from([
                        ("scifi_torso.glb", CharacterPartCategories::Torso),
                        ("scifi_head.glb", CharacterPartCategories::Head),
                        ("scifi_legs.glb", CharacterPartCategories::Leg),
                        ("sword.glb", CharacterPartCategories::Weapon),
                    ]),
                    CombatantSpecies::Wasp => {
                        Vec::from([("wasp_full.glb", CharacterPartCategories::FullBodyMesh)])
                    }
                    CombatantSpecies::Frog => {
                        Vec::from([("frog_full.glb", CharacterPartCategories::FullBodyMesh)])
                    }
                };
                for (part_name, category) in parts_to_spawn {
                    spawn_part(
                        &part_name.to_string(),
                        &category,
                        &mut commands,
                        &asset_pack,
                        &assets_gltf,
                        &mut parts_awaiting_spawn,
                    );
                }
            }
        }
    }

    for character_id in character_ids_of_skeletons_readied {
        skeletons_awaiting_combatant_assignment
            .0
            .remove(&character_id);
    }
}
