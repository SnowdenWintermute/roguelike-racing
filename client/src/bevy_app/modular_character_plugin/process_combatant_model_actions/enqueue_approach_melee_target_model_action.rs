use crate::bevy_app::modular_character_plugin::process_combatant_model_actions::model_actions::CombatantModelActions;
use crate::bevy_app::modular_character_plugin::process_combatant_model_actions::ModelActionQueue;
use crate::bevy_app::modular_character_plugin::process_combatant_model_actions::TransformManager;
use crate::bevy_app::modular_character_plugin::spawn_combatant::HitboxRadius;
use crate::bevy_app::modular_character_plugin::spawn_combatant::MainSkeletonEntity;
use bevy::prelude::*;
use common::combat::combat_actions::CombatActionTarget;
use common::combat::combat_actions::FriendOrFoe;
use common::combat::ActionResult;
use common::primatives::EntityId;
use std::collections::HashMap;

pub fn enqueue_approach_melee_target_model_action(
    action_result: &ActionResult,
    transform_manager: &mut TransformManager,
    model_action_queue: &mut ModelActionQueue,
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

    if let Some(target_id) = target_id_option {
        model_action_queue
            .0
            .push_back(CombatantModelActions::ApproachMeleeTarget);

        // set destination
        // get locations of combatant and target
        let target_entity = combatants_by_id
            .get(&target_id)
            .expect("to have the entity");
        let (target_skeleton_entity, target_hitbox_radius) = target_combatants
            .get(*target_entity)
            .expect("to have the combatant");

        let cloned_target_hitbox_radius = target_hitbox_radius.clone();
        let target_transformm = transforms
            .get(target_skeleton_entity.0)
            .expect("to have the transformm")
            .clone();
        let combatant_transformm = transforms
            .get(skeleton_entity)
            .expect("to have the transformm")
            .clone();

        // Calculate destination
        let direction =
            (combatant_transformm.translation - target_transformm.translation).normalize();
        let destination = target_transformm.translation + direction * cloned_target_hitbox_radius.0;
        transform_manager.destination = Some(Transform::from_xyz(
            destination[0],
            destination[1],
            destination[2],
        ));

        let up = *combatant_transformm.up().clone();
        transform_manager.target_rotation = Some(
            combatant_transformm
                .looking_at(
                    transform_manager
                        .destination
                        .expect("declared above")
                        .translation,
                    up,
                )
                .rotation,
        );
    }
}
