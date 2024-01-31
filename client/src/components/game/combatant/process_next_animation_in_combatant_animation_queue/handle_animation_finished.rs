use crate::components::game::combatant::process_next_animation_in_combatant_animation_queue::approach_combatant_animation_finished_handler::approach_combatant_animation_finished_handler;
use crate::components::game::combatant::process_next_animation_in_combatant_animation_queue::follow_through_swing_animation_finished_handler::follow_through_swing_animation_finished_handler;
use crate::components::game::combatant::process_next_animation_in_combatant_animation_queue::return_to_ready_position_animation_finished_handler::return_to_ready_position_animation_finished_handler;
use crate::components::game::combatant::process_next_animation_in_combatant_animation_queue::swing_to_hit_animation_finished_handler::swing_to_hit_animation_finished_handler;
use crate::components::mesh_manager::CombatantAnimation;
use crate::store::game_store::GameStore;
use common::app_consts::error_messages;
use common::errors::AppError;
use yewdux::Dispatch;

pub fn handle_animation_finished(
    game_dispatch: Dispatch<GameStore>,
    animation: CombatantAnimation,
    combatant_id: u32,
) -> Result<(), AppError> {
    match animation {
        CombatantAnimation::SwingMainHandToHit(target_id, hp_change_option, evaded) => {
            swing_to_hit_animation_finished_handler(
                game_dispatch.clone(),
                target_id,
                hp_change_option,
                evaded,
                combatant_id,
            )
        }
        CombatantAnimation::SwingOffHandToHit => Ok(()),
        CombatantAnimation::MainHandFollowThroughSwing => {
            follow_through_swing_animation_finished_handler(game_dispatch.clone(), combatant_id)
        }
        CombatantAnimation::OffHandFollowThroughSwing => todo!(),
        CombatantAnimation::ReturnToReadyPosition => {
            return_to_ready_position_animation_finished_handler(game_dispatch.clone(), combatant_id)
        }
        CombatantAnimation::HitRecovery(_) => Ok(()),
        CombatantAnimation::Death(_) => Ok(()),
        CombatantAnimation::TurnToFaceCombatant(_) => Ok(()),
        CombatantAnimation::ApproachCombatant(target_id) => {
            approach_combatant_animation_finished_handler(
                game_dispatch.clone(),
                combatant_id,
                target_id,
            )
        }
        CombatantAnimation::Evasion => Ok(()),
        CombatantAnimation::UseAutoinjector(_, _) => todo!(),
    }?;

    game_dispatch.reduce_mut(|store| -> Result<(), AppError> {
        let event_manager = store
            .action_results_manager
            .combantant_event_managers
            .get_mut(&combatant_id)
            .ok_or_else(|| AppError {
                error_type: common::errors::AppErrorTypes::ClientError,
                message: error_messages::COMBANTANT_EVENT_MANAGER_NOT_FOUND.to_string(),
            })?;
        event_manager.animation_queue.pop_front();

        Ok(())
    })
}
