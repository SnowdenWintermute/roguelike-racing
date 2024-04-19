use super::model_actions::CombatantModelActions;
use super::process_active_model_actions::ModelActionSystemParams;
use crate::bevy_app::modular_character_plugin::StartNextModelActionEvent;
use crate::bevy_app::utils::rotate_transform_toward_target::rotate_transform_toward_target;
use crate::bevy_app::utils::translate_transform_toward_target::translate_transform_toward_target;
use bevy::math::u64;
use bevy::prelude::*;

const TIME_TO_TRANSLATE: u64 = 1500;
const TIME_TO_ROTATE: u64 = 1000;
const PERCENT_DISTANCE_TO_START_TRANSITION: f32 = 0.8;

pub fn combatant_approaching_melee_target_processor(
    entity: Entity,
    elapsed: u64,
    transition_started: bool,
    model_action_params: &mut ModelActionSystemParams,
    model_actions_to_remove: &mut Vec<CombatantModelActions>,
    start_next_model_action_event_writer: &mut EventWriter<StartNextModelActionEvent>,
) {
    let (
        _,
        _,
        _,
        _,
        skeleton_entity,
        _,
        home_location,
        _,
        _,
        _,
        transform_manager,
        _,
        mut active_model_actions,
    ) = model_action_params
        .combatants_query
        .get_mut(entity)
        .expect("to have this entity in the query");
    let mut skeleton_entity_transform = model_action_params
        .transforms
        .get_mut(skeleton_entity.0)
        .expect("their skeleton to have a transform");
    // move toward destination
    let percent_distance_travelled = translate_transform_toward_target(
        &mut skeleton_entity_transform,
        &home_location.0,
        &mut transform_manager.destination.expect("a destination"),
        elapsed,
        TIME_TO_TRANSLATE,
    );
    if let Some(target_rotation) = transform_manager.target_rotation {
        rotate_transform_toward_target(
            &mut skeleton_entity_transform,
            &home_location.0.rotation,
            &target_rotation,
            elapsed,
            TIME_TO_ROTATE,
        );
    }

    if percent_distance_travelled >= PERCENT_DISTANCE_TO_START_TRANSITION && !transition_started {
        // start next model action and mark this one's transition as started
        start_next_model_action_event_writer.send(StartNextModelActionEvent {
            entity,
            transition_duration_ms: 500,
        });

        active_model_actions
            .0
            .get_mut(&CombatantModelActions::ApproachMeleeTarget)
            .expect("this model action to be active")
            .transition_started = true;
    }
    // - if reached destination, deactivate approaching
    if percent_distance_travelled >= 1.0 {
        model_actions_to_remove.push(CombatantModelActions::ApproachMeleeTarget);
    }
}
