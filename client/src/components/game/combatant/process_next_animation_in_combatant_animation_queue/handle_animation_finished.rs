use crate::components::game::combatant::process_next_animation_in_combatant_animation_queue::handle_follow_through_swing_animation_finished::handle_follow_through_swing_animation_finished;
use crate::components::game::combatant::process_next_animation_in_combatant_animation_queue::handle_return_to_ready_position_animation_finished::handle_return_to_ready_position_animation_finished;
use crate::components::game::combatant::process_next_animation_in_combatant_animation_queue::handle_swing_to_hit_animation_finished::handle_swing_to_hit_animation_finished;
use crate::components::mesh_manager::CombatantAnimation;
use crate::store::game_store::GameStore;
use common::app_consts::error_messages;
use common::errors::AppError;
use gloo::console::log;
use yewdux::Dispatch;

pub fn handle_animation_finished(
    game_dispatch: Dispatch<GameStore>,
    animation: CombatantAnimation,
    combatant_id: u32,
) -> Result<(), AppError> {
    log!(format!("{} finished", animation));
    match animation {
        CombatantAnimation::SwingMainHandToHit(target_id, hp_change_option) => {
            handle_swing_to_hit_animation_finished(
                game_dispatch.clone(),
                target_id,
                hp_change_option,
                combatant_id,
            )
        }
        CombatantAnimation::SwingOffHandToHit => todo!(),
        CombatantAnimation::MainHandFollowThroughSwing => {
            handle_follow_through_swing_animation_finished(game_dispatch.clone())
        }
        CombatantAnimation::OffHandFollowThroughSwing => todo!(),
        CombatantAnimation::ReturnToReadyPosition => {
            handle_return_to_ready_position_animation_finished(game_dispatch.clone())
        }
        CombatantAnimation::HitRecovery(_) => todo!(),
        CombatantAnimation::Death(_) => todo!(),
        CombatantAnimation::TurnToFaceCombatant(_) => todo!(),
        CombatantAnimation::ApproachCombatant(_) => todo!(),
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
