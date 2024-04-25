use super::model_actions::CombatantModelActions;
use super::process_active_model_actions::ModelActionCombatantQueryStructItem;
use super::process_active_model_actions::ModelActionSystemParams;
use crate::bevy_app::utils::rotate_transform_toward_target::rotate_transform_toward_target;
use bevy::prelude::*;

pub const TIME_TO_RECENTER: u64 = 1000;

pub fn combatant_recentering_processor(
    entity: Entity,
    elapsed: u64,
    model_action_params: &mut ModelActionSystemParams,
) {
    let ModelActionCombatantQueryStructItem {
        skeleton_entity,
        home_location,
        transform_manager,
        mut active_model_actions,
        ..
    } = model_action_params
        .combatants_query
        .get_mut(entity)
        .expect("to have this entity in the query");
    let mut skeleton_entity_transform = model_action_params
        .transforms
        .get_mut(skeleton_entity.0)
        .expect("their skeleton to have a transform");
    let target_rotation = match &transform_manager.target_rotation {
        Some(rotation) => rotation,
        None => &home_location.0.rotation,
    };
    let percent_rotated = rotate_transform_toward_target(
        &mut skeleton_entity_transform,
        &transform_manager.last_location.rotation,
        target_rotation,
        elapsed,
        TIME_TO_RECENTER,
    );

    if percent_rotated >= 1.0 {
        active_model_actions
            .0
            .remove(&CombatantModelActions::Recenter);
    }
}
