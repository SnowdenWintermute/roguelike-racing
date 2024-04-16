use bevy::prelude::*;

use crate::bevy_app::modular_character_plugin::spawn_combatant::CombatantActionResultsManagerComponent;

pub fn start_processing_next_action_results(
    mut combatant_action_result_managers: Query<
        &mut CombatantActionResultsManagerComponent,
        Changed<CombatantActionResultsManagerComponent>,
    >,
) {
    for mut action_result_manager in &mut combatant_action_result_managers {
        if action_result_manager
            .current_action_result_processing
            .is_some()
            || action_result_manager.action_result_queue.len() < 1
        {
            continue;
        }
        action_result_manager.current_action_result_processing =
            action_result_manager.action_result_queue.pop_front();
        info!("new current action result");
    }
}
