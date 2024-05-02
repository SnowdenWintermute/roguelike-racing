use super::start_processing_new_action_results::start_processing_new_action_results;
use super::ModelActionQueue;
use super::TransformManager;
use crate::bevy_app::modular_character_plugin::spawn_combatant::ActionResultsProcessing;
use crate::bevy_app::modular_character_plugin::spawn_combatant::HitboxRadius;
use crate::bevy_app::modular_character_plugin::spawn_combatant::MainSkeletonEntity;
use crate::bevy_app::modular_character_plugin::CombatantsById;
use crate::bevy_app::modular_character_plugin::RawActionResultsQueue;
use crate::comm_channels::BevyTransmitter;
use bevy::prelude::*;

pub fn process_new_raw_action_results_handler(
    mut new_action_results: ResMut<RawActionResultsQueue>,
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
    // enqueue model actions from action result
    while let Some((action_user_id, action_results)) = new_action_results.0.pop_front() {
        start_processing_new_action_results(
            &combatants_by_id,
            &bevy_transmitter,
            action_user_id,
            &mut combatants,
            &target_combatants,
            &transforms,
            action_results,
            false,
        );
    }
}
