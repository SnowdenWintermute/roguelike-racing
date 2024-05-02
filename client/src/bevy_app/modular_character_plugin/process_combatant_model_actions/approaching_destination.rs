use super::model_actions::CombatantModelActions;
use super::process_active_model_actions::ModelActionCombatantQueryStructItem;
use super::process_active_model_actions::ModelActionSystemParams;
use crate::bevy_app::modular_character_plugin::StartNextModelActionEvent;
use crate::bevy_app::utils::rotate_transform_toward_target::rotate_transform_toward_target;
use crate::bevy_app::utils::translate_transform_toward_target::translate_transform_toward_target;
use bevy::math::u64;
use bevy::prelude::*;

const PERCENT_DISTANCE_TO_START_TRANSITION: f32 = 0.8;

pub fn combatant_approaching_destination_processor(
    entity: Entity,
    elapsed: u64,
    transition_started: bool,
    model_action_params: &mut ModelActionSystemParams,
    start_next_model_action_event_writer: &mut EventWriter<StartNextModelActionEvent>,
) {
    let ModelActionCombatantQueryStructItem {
        skeleton_entity,
        home_location,
        mut transform_manager,
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

    let destination = &mut transform_manager.destination.expect("a destination");

    let percent_distance_travelled = translate_transform_toward_target(
        &mut skeleton_entity_transform,
        &transform_manager.last_location,
        destination,
        elapsed,
        transform_manager.time_to_translate,
    );
    if let Some(target_rotation) = transform_manager.target_rotation {
        rotate_transform_toward_target(
            &mut skeleton_entity_transform,
            &home_location.0.rotation,
            &target_rotation,
            elapsed,
            transform_manager.time_to_rotate,
        );
    }

    if percent_distance_travelled >= PERCENT_DISTANCE_TO_START_TRANSITION && !transition_started {
        start_next_model_action_event_writer.send(StartNextModelActionEvent {
            entity,
            transition_duration_ms: 500,
        });

        active_model_actions
            .0
            .get_mut(&CombatantModelActions::ApproachDestination)
            .expect("this model action to be active")
            .transition_started = true;
    }

    if percent_distance_travelled >= 1.0 {
        let combatant_transform = model_action_params
            .transforms
            .get(skeleton_entity.0)
            .expect("to have the transform");
        info!(
            "setting destination, current translation: {:?}, home_location translation :{:?}",
            combatant_transform.translation, home_location.0.translation
        );
        transform_manager
            .set_destination(combatant_transform.clone(), Some(home_location.0.clone()));

        active_model_actions
            .0
            .remove(&CombatantModelActions::ApproachDestination);
    }
}
