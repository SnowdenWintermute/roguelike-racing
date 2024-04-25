use crate::bevy_app::modular_character_plugin::process_combatant_model_actions::TransformManager;
use crate::bevy_app::modular_character_plugin::spawn_combatant::HitboxRadius;
use crate::bevy_app::modular_character_plugin::spawn_combatant::MainSkeletonEntity;
use bevy::prelude::*;
use common::combat::combat_actions::CombatActionTarget;
use common::combat::combat_actions::FriendOrFoe;
use common::combat::ActionResult;
use common::primatives::EntityId;
use std::collections::HashMap;

pub fn set_non_melee_ability_destination_transform_and_rotation(
    action_result: &ActionResult,
    transform_manager: &mut TransformManager,
    combatants_by_id: &HashMap<EntityId, Entity>,
    skeleton_entity: Entity,
    target_combatants: &Query<(&MainSkeletonEntity, &HitboxRadius)>,
    transforms: &Query<&Transform>,
) {
    let target_id_option = match &action_result.targets {
        CombatActionTarget::Single(combatant_id) => Some(combatant_id),
        CombatActionTarget::Group(group) => match group {
            FriendOrFoe::Friendly => None,
            FriendOrFoe::Hostile => None,
        },
        CombatActionTarget::All => None,
    };

    let combatant_transform = transforms
        .get(skeleton_entity)
        .expect("to have the transformm")
        .clone();
    let up = *combatant_transform.up().clone();
    let direction = combatant_transform.forward();
    let destination_vec = combatant_transform.translation + direction * 1.5;
    let (destination_transform, _) = if let Some(target_id) = target_id_option {
        // set destination
        // get locations of combatant and target
        let target_entity = combatants_by_id
            .get(&target_id)
            .expect("to have the entity");
        let (target_skeleton_entity, _) = target_combatants
            .get(*target_entity)
            .expect("to have the combatant");

        let target_transform = transforms
            .get(target_skeleton_entity.0)
            .expect("to have the transformm")
            .clone();

        // Calculate destination
        (
            Some(Transform::from_translation(destination_vec)),
            combatant_transform
                .looking_at(target_transform.translation, up)
                .rotation,
        )
    } else {
        (
            Some(Transform::from_translation(destination_vec)),
            combatant_transform.rotation,
        )
    };

    transform_manager.set_destination(combatant_transform.clone(), destination_transform);
    transform_manager.set_target_rotation(Some(combatant_transform.rotation));
}
