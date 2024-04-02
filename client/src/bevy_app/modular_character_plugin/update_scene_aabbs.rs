// taken from
// https://gist.github.com/dmlary/b9e1e9ef18789dfb0e6df8aca2f1ed74#file-scene_aabb-rs-L268
//
/*
      (2)-----(3)               Y
       | \     | \              |
       |  (1)-----(0) MAX       o---X
       |   |   |   |             \
  MIN (6)--|--(7)  |              Z
         \ |     \ |
          (5)-----(4)
*/
use super::spawn_combatant::CombatantMainArmatureMarker;
use bevy::prelude::*;
use bevy::render::primitives::Aabb;

#[derive(Component, Debug, Reflect)]
pub struct SceneAabb {
    pub min: Vec3,
    pub max: Vec3,
}

impl SceneAabb {
    fn new(center: Vec3) -> Self {
        Self {
            min: center,
            max: center,
        }
    }

    /// merge a child AABB into the Scene AABB
    fn merge_aabb(&mut self, aabb: &Aabb, global_transform: &GlobalTransform) {
        let min = aabb.min();
        let max = aabb.max();
        // info!(format!("merging child aabb"));
        let corners = [
            global_transform.transform_point(Vec3::new(max.x, max.y, max.z)),
            global_transform.transform_point(Vec3::new(min.x, max.y, max.z)),
            global_transform.transform_point(Vec3::new(min.x, max.y, min.z)),
            global_transform.transform_point(Vec3::new(max.x, max.y, min.z)),
            global_transform.transform_point(Vec3::new(max.x, min.y, max.z)),
            global_transform.transform_point(Vec3::new(min.x, min.y, max.z)),
            global_transform.transform_point(Vec3::new(min.x, min.y, min.z)),
            global_transform.transform_point(Vec3::new(max.x, min.y, min.z)),
        ];

        for corner in corners {
            let gt = corner.cmpgt(self.max);
            let lt = corner.cmplt(self.min);

            debug!("corner {:?}, gt {:?}, lt {:?}", corner, lt, gt);

            if gt.x {
                self.max.x = corner.x;
            } else if lt.x {
                self.min.x = corner.x;
            }

            if gt.y {
                self.max.y = corner.y;
            } else if lt.y {
                self.min.y = corner.y;
            }

            if gt.z {
                self.max.z = corner.z;
            } else if lt.z {
                self.min.z = corner.z;
            }
        }

        // debug!("min {:?}, max {:?}", min, max);
    }
}

pub fn update_scene_aabbs_on_changed_children(
    mut commands: Commands,
    entities_with_changed_children: Query<
        (Entity, &GlobalTransform, &CombatantMainArmatureMarker),
        Changed<Children>,
    >,
    children: Query<&Children>,
    bounding_boxes_and_global_transforms: Query<(&Aabb, &GlobalTransform)>,
) {
    for (entity, global_transform, _) in entities_with_changed_children.iter() {
        update_scene_aabb(
            &mut commands,
            entity,
            &global_transform,
            &children,
            &bounding_boxes_and_global_transforms,
        );
    }
}

pub fn update_scene_aabb(
    commands: &mut Commands,
    entity: Entity,
    global_transform: &GlobalTransform,
    children: &Query<&Children>,
    bounding_boxes_and_global_transforms: &Query<(&Aabb, &GlobalTransform)>,
) {
    let mut scene_aabb = SceneAabb::new(global_transform.translation());

    for child in children.iter_descendants(entity) {
        let Ok((bb, transform)) = bounding_boxes_and_global_transforms.get(child) else {
            continue;
        };
        scene_aabb.merge_aabb(bb, transform);
    }

    // info!(format!("Scene Entity {:?}, AABB {:?}", entity, scene_aabb));
    commands.entity(entity).insert(scene_aabb);
}
