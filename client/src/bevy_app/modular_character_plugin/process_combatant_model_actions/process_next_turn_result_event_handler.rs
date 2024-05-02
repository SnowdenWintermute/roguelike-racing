use super::start_processing_new_action_results::start_processing_new_action_results;
use super::ModelActionQueue;
use super::TransformManager;
use crate::bevy_app::modular_character_plugin::spawn_combatant::ActionResultsProcessing;
use crate::bevy_app::modular_character_plugin::spawn_combatant::HitboxRadius;
use crate::bevy_app::modular_character_plugin::spawn_combatant::MainSkeletonEntity;
use crate::bevy_app::modular_character_plugin::CombatantsById;
use crate::bevy_app::modular_character_plugin::TurnResultsQueue;
use crate::comm_channels::messages_from_bevy::MessageFromBevy;
use crate::comm_channels::BevyTransmitter;
use crate::comm_channels::ProcessNextTurnResultEvent;
use bevy::prelude::*;
use common::combat::CombatTurnResult;

pub fn process_next_turn_result_event_handler(
    mut proccess_next_turn_result_event_reader: EventReader<ProcessNextTurnResultEvent>,
    mut turn_results_queue: ResMut<TurnResultsQueue>,
    combatants_by_id: Res<CombatantsById>,
    mut combatants: Query<(
        &mut TransformManager,
        &mut ModelActionQueue,
        &mut ActionResultsProcessing,
        &MainSkeletonEntity,
    )>,
    target_combatants: Query<(&MainSkeletonEntity, &HitboxRadius)>,
    transforms: Query<&Transform>,
    bevy_transmitter: Res<BevyTransmitter>,
) {
    for event in proccess_next_turn_result_event_reader.read() {
        if let Some(combatant_id) = event.0 {
            let _result = bevy_transmitter
                .0
                .send(MessageFromBevy::FinishedProcessingTurnResult(combatant_id));
        }

        if let Some(turn_result) = turn_results_queue.0.pop_front() {
            let CombatTurnResult {
                combatant_id,
                action_results,
            } = turn_result;

            start_processing_new_action_results(
                &combatants_by_id,
                &bevy_transmitter,
                combatant_id,
                &mut combatants,
                &target_combatants,
                &transforms,
                action_results,
                true,
            )
        }
    }
}
