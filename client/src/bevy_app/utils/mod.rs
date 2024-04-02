pub mod collect_hierarchy;
pub mod find_child_with_name_containing;
pub mod link_animations;
mod mark_scenes_as_loaded;
pub mod paint_cubes_on_scene_children;
pub mod print_scene_tree;

use bevy::prelude::*;
pub fn zero_transform(entity: Entity, transforms: &mut Query<&mut Transform>) {
    if let Ok(mut transform) = transforms.get_mut(entity) {
        transform.translation.x = 0.0;
        transform.translation.y = 0.0;
        transform.translation.z = 0.0;
        transform.rotation = Quat::from_xyzw(0.0, 0.0, 0.0, 0.0);
    }
}
