use crate::bevy_app::modular_character_plugin::spawn_combatant::CombatantActionResultsManagerComponent;
use crate::bevy_app::modular_character_plugin::CombatantsById;
use crate::bevy_app::modular_character_plugin::CurrentTurnResultProcessing;
use crate::bevy_app::modular_character_plugin::TurnResultsQueue;
use bevy::prelude::*;

pub fn start_processing_next_turn_result_in_queue(
    mut turn_results_queue: ResMut<TurnResultsQueue>,
    mut current_turn_result_processing: ResMut<CurrentTurnResultProcessing>,
    mut combatant_action_result_managers: Query<&mut CombatantActionResultsManagerComponent>,
    combatants_by_id: Res<CombatantsById>,
) {
    if current_turn_result_processing.0.is_some() || turn_results_queue.0.len() < 1 {
        return;
    }
    current_turn_result_processing.0 = turn_results_queue.0.pop_front();

    // take the actions in the turn result and send them to the combatant's action results manager
    for action_result in &current_turn_result_processing
        .0
        .as_ref()
        .expect("assigned above")
        .action_results
    {
        if let Some(action_user_entity) = combatants_by_id.0.get(&action_result.user_id) {
            if let Ok(mut action_result_manager) =
                combatant_action_result_managers.get_mut(*action_user_entity)
            {
                action_result_manager
                    .action_result_queue
                    .push_back(action_result.clone());
            };
        } else {
            error!("error - no bevy combatant found matching the user id in the action result")
        };
    }
}
