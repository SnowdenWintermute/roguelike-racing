use crate::bevy_app::utils::collect_hierarchy::get_all_named_entities_in_hierarchy;
use crate::bevy_app::utils::find_child_with_name_containing::find_child_with_name_containing;
use bevy::prelude::*;
use bevy::utils::HashMap;

pub fn get_main_skeleton_bones_and_armature(
    main_skeleton_entity: &Entity,
    all_entities_with_children: &Query<&Children>,
    names: &Query<&Name>,
) -> (HashMap<String, Entity>, Entity) {
    let root_bone = find_child_with_name_containing(
        all_entities_with_children,
        &names,
        &main_skeleton_entity,
        "Root",
    )
    .expect("the skeleton to have a bone called 'Root'");

    let main_skeleton_armature = find_child_with_name_containing(
        all_entities_with_children,
        &names,
        &main_skeleton_entity,
        "Armature",
    )
    .expect("the skeleton to have an armature");

    let main_bones =
        get_all_named_entities_in_hierarchy(&all_entities_with_children, &names, &root_bone);

    (main_bones, main_skeleton_armature)
}
