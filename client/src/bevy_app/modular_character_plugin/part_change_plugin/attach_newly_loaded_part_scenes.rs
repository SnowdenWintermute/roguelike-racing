use crate::{
    bevy_app::modular_character_plugin::{
        assemble_parts::{
            attach_holdable::attach_holdable,
            attach_part_to_main_skeleton::attach_part_to_main_skeleton,
        },
        part_change_plugin::despawn_attached_part::despawn_attached_part,
        spawn_combatant::{
            CharacterAttachedPartScenes, CombatantIdComponent, CharacterPartScenesAwaitingSpawn,
            MainSkeletonBonesAndArmature, MainSkeletonEntity,
        },
        spawn_scenes::{SceneLoaded, SceneName},
        AttachedPartsReparentedEntities,
    },
    frontend_common::CharacterPartCategories,
};
use bevy::{prelude::*, scene::SceneInstance};

pub fn attach_newly_loaded_part_scenes(
    mut commands: Commands,
    scene_manager: Res<SceneSpawner>,
    unloaded_instances: Query<(Entity, &SceneInstance, &SceneName), Without<SceneLoaded>>,
    mut character_query: Query<(
        Entity,
        &CombatantIdComponent,
        &MainSkeletonEntity,
        &MainSkeletonBonesAndArmature,
        &mut CharacterPartScenesAwaitingSpawn,
        &mut CharacterAttachedPartScenes,
    )>,
    mut attached_parts_reparented_entities: ResMut<AttachedPartsReparentedEntities>,
    all_entities_with_children: Query<&Children>,
    mut transforms: Query<&mut Transform>,
    names: Query<&Name>,
    mut visibility_query: Query<&mut Visibility>,
) {
    //  - for each character's list of parts awaiting spawn, check newly loaded scenes
    for (
        _,
        _,
        _,
        main_skeleton_bones_and_armature,
        mut parts_waiting_to_spawn,
        mut attached_parts,
    ) in character_query.iter_mut()
    {
        let mut spawned_parts = Vec::new();
        // check for parts awaiting spawn
        for (category, part_entities) in parts_waiting_to_spawn.0.iter() {
            for part_entity in part_entities {
                if let Ok((entity, instance, _)) = unloaded_instances.get(*part_entity) {
                    if !scene_manager.instance_is_ready(**instance) {
                        continue;
                    }
                    // mark as loaded
                    commands.entity(*part_entity).insert(SceneLoaded);
                    spawned_parts.push((category.clone(), *part_entity));

                    //    - despawn any currently attached part in that category
                    if let Some(old_part) = attached_parts.0.remove(category) {
                        despawn_attached_part(
                            &mut commands,
                            &old_part,
                            &mut attached_parts_reparented_entities,
                        );
                    };
                    //    - add newly spawned part to character's list of attached parts
                    attached_parts.0.insert(category.clone(), entity);
                    //    - attach newly spawned part to character's skeleton bones
                    //    - make character armature visible
                    match category {
                        CharacterPartCategories::Weapon => attach_holdable(
                            &mut commands,
                            &entity,
                            &main_skeleton_bones_and_armature.0,
                            &mut visibility_query,
                        ),
                        _ => {
                            attach_part_to_main_skeleton(
                                &mut commands,
                                &all_entities_with_children,
                                &mut transforms,
                                &names,
                                &entity,
                                &main_skeleton_bones_and_armature.1,
                                &main_skeleton_bones_and_armature.0,
                                &mut attached_parts_reparented_entities,
                                &mut visibility_query,
                            );
                        }
                    }
                }
            }
        }

        // remove from character's list of parts awaiting spawn
        for (category, spawned_part_entity) in spawned_parts {
            let parts_in_category = parts_waiting_to_spawn
                .0
                .get_mut(&category)
                .expect("to have this category");
            parts_in_category.remove(&spawned_part_entity);
        }
    }
}
