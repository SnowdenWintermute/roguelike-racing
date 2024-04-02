use bevy::{prelude::*, utils::HashMap};

pub fn attach_holdable(
    commands: &mut Commands,
    holdable_scene_entity: &Entity,
    main_skeleton_bones: &HashMap<String, Entity>,
    visibility_query: &mut Query<&mut Visibility>,
) {
    let handle_bone = main_skeleton_bones
        .get("EquipmentHandle.R")
        .expect("to have an equipment handle bone");
    // zero_transform(holdable_scene_entity, transforms);
    // set visibility
    if let Ok(mut visibility) = visibility_query.get_mut(*holdable_scene_entity) {
        *visibility = Visibility::Visible;
    }

    let mut entity_commands = commands.entity(*holdable_scene_entity);

    entity_commands.set_parent(*handle_bone);
}
