use super::combatant_model_actions::CombatantModelActions;
use crate::bevy_app::modular_character_plugin::animation_manager_component::AnimationManagerComponent;
use crate::bevy_app::modular_character_plugin::spawn_combatant::HitboxRadius;
use crate::bevy_app::modular_character_plugin::spawn_combatant::MainSkeletonEntity;
use crate::bevy_app::modular_character_plugin::CombatantId;
use bevy::prelude::*;
use common::combat::combat_actions::CombatActionTarget;
use common::combat::combat_actions::FriendOrFoe;
use common::combat::ActionResult;
use std::collections::HashMap;

pub fn enqueue_approach_melee_target_model_action(
    current_action_result_processing: &ActionResult,
    animation_manager: &mut AnimationManagerComponent,
    combatants_by_id: &HashMap<CombatantId, Entity>,
    skeleton_entity: Entity,
    target_combatants: &Query<(&MainSkeletonEntity, &HitboxRadius)>,
    transforms: &Query<&Transform>,
) {
    let target_id_option = match &current_action_result_processing.targets {
        CombatActionTarget::Single(combatant_id) => Some(combatant_id),
        CombatActionTarget::Group(group) => match group {
            FriendOrFoe::Friendly => None,
            FriendOrFoe::Hostile => None,
        },
        CombatActionTarget::All => None,
    };

    info!("target id option: {:?}", target_id_option);

    if let Some(target_id) = target_id_option {
        animation_manager
            .model_action_queue
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
        let target_transform = transforms
            .get(target_skeleton_entity.0)
            .expect("to have the transform")
            .clone();
        let combatant_transform = transforms
            .get(skeleton_entity)
            .expect("to have the transform")
            .clone();

        // Calculate destination
        let direction =
            (combatant_transform.translation - target_transform.translation).normalize();
        let destination = target_transform.translation + direction * cloned_target_hitbox_radius.0;
        info!("destination: {:?}", destination);
        animation_manager.destination = Some(Transform::from_xyz(
            destination[0],
            destination[1],
            destination[2],
        ));

        let up = *combatant_transform.up().clone();
        animation_manager.target_rotation = Some(
            combatant_transform
                .looking_at(
                    animation_manager
                        .destination
                        .expect("declared above")
                        .translation,
                    up,
                )
                .rotation,
        );
    }
}
