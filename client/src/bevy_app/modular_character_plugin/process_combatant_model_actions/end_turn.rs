use super::model_actions::CombatantModelActions;
use super::process_active_model_actions::ModelActionCombatantQueryStructItem;
use super::process_active_model_actions::ModelActionSystemParams;
use crate::comm_channels::ProcessNextTurnResultEvent;
use bevy::prelude::*;

pub fn process_combatant_ending_turn(
    entity: Entity,
    process_next_turn_result_event_writer: &mut EventWriter<ProcessNextTurnResultEvent>,
    model_action_params: &mut ModelActionSystemParams,
) {
    let ModelActionCombatantQueryStructItem {
        mut active_model_actions,
        combatant_id_component,
        ..
    } = model_action_params
        .combatants_query
        .get_mut(entity)
        .expect("to have this entity in the query");
    active_model_actions
        .0
        .remove(&CombatantModelActions::EndTurn);
    process_next_turn_result_event_writer
        .send(ProcessNextTurnResultEvent(Some(combatant_id_component.0)));
}
