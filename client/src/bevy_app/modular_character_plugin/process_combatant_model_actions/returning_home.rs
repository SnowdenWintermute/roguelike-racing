use super::model_actions::CombatantModelActions;
use super::process_active_model_actions::ModelActionCombatantQueryStructItem;
use super::process_active_model_actions::ModelActionSystemParams;
use crate::bevy_app::modular_character_plugin::StartNextModelActionEvent;
use crate::bevy_app::utils::translate_transform_toward_target::translate_transform_toward_target;
use crate::comm_channels::ProcessNextTurnResultEvent;
use bevy::prelude::*;

const PERCENT_DISTANCE_TO_START_IDLE: f32 = 0.8;

pub fn combatant_returning_to_home_position_home_processor(
    entity: Entity,
    elapsed: u64,
    transition_started: bool,
    model_action_params: &mut ModelActionSystemParams,
    start_next_model_action_event_writer: &mut EventWriter<StartNextModelActionEvent>,
    process_next_turn_result_event_writer: &mut EventWriter<ProcessNextTurnResultEvent>,
) {
    let ModelActionCombatantQueryStructItem {
        combatant_id_component,
        skeleton_entity,
        mut transform_manager,
        home_location,
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

    let percent_distance_travelled = translate_transform_toward_target(
        &mut skeleton_entity_transform,
        &transform_manager.last_location,
        &transform_manager
            .destination
            .expect("to have set the destination"),
        elapsed,
        transform_manager.time_to_translate,
    );

    if percent_distance_travelled >= PERCENT_DISTANCE_TO_START_IDLE && !transition_started {
        start_next_model_action_event_writer.send(StartNextModelActionEvent {
            entity,
            transition_duration_ms: 500,
        });
        active_model_actions
            .0
            .get_mut(&CombatantModelActions::ReturnHome)
            .expect("this model action to be active")
            .transition_started = true;
    }

    if percent_distance_travelled >= 1.0 {
        active_model_actions
            .0
            .remove(&CombatantModelActions::ReturnHome);

        transform_manager.set_target_rotation(Some(home_location.0.rotation));
        transform_manager.set_destination(skeleton_entity_transform.clone(), Some(home_location.0));

        process_next_turn_result_event_writer
            .send(ProcessNextTurnResultEvent(Some(combatant_id_component.0)));
    }
}
