use crate::bevy_app::modular_character_plugin::spawn_combatant::CombatantActionResultManagerComponent;
use bevy::prelude::*;

pub fn start_processing_next_action_results(
    mut combatant_action_result_managers: Query<
        &mut CombatantActionResultManagerComponent,
        Changed<CombatantActionResultManagerComponent>,
    >,
) {
    for mut action_result_manager in &mut combatant_action_result_managers {
        if action_result_manager
            .0
            .current_action_result_processing
            .is_some()
            || action_result_manager.0.action_result_queue.len() < 1
        {
            continue;
        }
        action_result_manager.0.current_action_result_processing =
            action_result_manager.0.action_result_queue.pop_front();
    }
}
