use crate::components::game::combatant::process_next_animation_in_combatant_animation_queue::approach_combatant_animation_finished_handler::approach_combatant_animation_finished_handler;
use crate::components::game::combatant::process_next_animation_in_combatant_animation_queue::follow_through_swing_animation_finished_handler::follow_through_swing_animation_finished_handler;
use crate::components::game::combatant::process_next_animation_in_combatant_animation_queue::return_to_ready_position_animation_finished_handler::return_to_ready_position_animation_finished_handler;
use crate::components::mesh_manager::CombatantAnimation;
use crate::store::game_store::GameStore;
use common::app_consts::error_messages;
use common::errors::AppError;
use yewdux::Dispatch;
use super::animation_causing_hp_change_finished_handler::animation_causing_hp_change_finished_handler;
use super::animation_causing_payment_of_mp_price_finished_handler::animation_causing_payment_of_mp_price_finished_handler;
use super::autoinjector_use_animation_finished_handler::autoinjector_use_animation_finished_handler;

pub fn handle_animation_finished(
    game_dispatch: Dispatch<GameStore>,
    animation: CombatantAnimation,
    combatant_id: u32,
) -> Result<(), AppError> {
    match animation {
        CombatantAnimation::SwingMainHandToHit(targets_and_hp_change_results) => {
            animation_causing_hp_change_finished_handler(
                game_dispatch.clone(),
                targets_and_hp_change_results,
                combatant_id,
            )
        }
        CombatantAnimation::SwingOffHandToHit => Ok(()),
        CombatantAnimation::MainHandFollowThroughSwing => {
            follow_through_swing_animation_finished_handler(game_dispatch.clone(), combatant_id)
        }
        CombatantAnimation::OffHandFollowThroughSwing => todo!(),
        CombatantAnimation::ReturnToReadyPosition(ends_turn) => {
            return_to_ready_position_animation_finished_handler(
                game_dispatch.clone(),
                combatant_id,
                ends_turn,
            )
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
        CombatantAnimation::UseAutoinjector(autoinjector_type, target_id, value_change) => {
            autoinjector_use_animation_finished_handler(
                game_dispatch.clone(),
                autoinjector_type,
                value_change,
                combatant_id,
                target_id,
            )
        }
        CombatantAnimation::CastSpellOnTargets(targets_and_hp_change_results) => {
            animation_causing_hp_change_finished_handler(
                game_dispatch.clone(),
                targets_and_hp_change_results,
                combatant_id,
            )
        }
        CombatantAnimation::MoveForwardToCastSpell(mp_price) => {
            animation_causing_payment_of_mp_price_finished_handler(
                game_dispatch.clone(),
                mp_price,
                combatant_id,
            )
        }
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
        let _ = event_manager.animation_queue.pop_front();

        Ok(())
    })
}
