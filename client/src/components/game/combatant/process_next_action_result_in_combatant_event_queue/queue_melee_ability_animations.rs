use crate::components::mesh_manager::CombatantAnimation;
use crate::components::mesh_manager::CombatantVisualLocation;
use crate::store::game_store::GameStore;
use common::app_consts::error_messages;
use common::combat::ActionResult;
use common::combatants::abilities::AbilityTarget;
use common::errors::AppError;
use std::collections::VecDeque;
use yewdux::Dispatch;

pub fn queue_melee_ability_animations(
    game_dispatch: Dispatch<GameStore>,
    combatant_id: u32,
    action_result: &ActionResult,
) -> Result<(), AppError> {
    let target_id = match action_result.targets {
        AbilityTarget::Single(id) => id,
        AbilityTarget::Group(_) => combatant_id, // @TODO
        AbilityTarget::All => combatant_id,
    };
    game_dispatch.reduce_mut(|store| -> Result<(), AppError> {
        let event_manager = store
            .action_results_manager
            .combantant_event_managers
            .get_mut(&combatant_id)
            .ok_or_else(|| AppError {
                error_type: common::errors::AppErrorTypes::ClientError,
                message: error_messages::COMBANTANT_EVENT_MANAGER_NOT_FOUND.to_string(),
            })?;

        match event_manager.visual_location {
            CombatantVisualLocation::HomePosition => {
                event_manager.animation_queue.append(&mut VecDeque::from([
                    CombatantAnimation::TurnToFaceCombatant(target_id),
                    CombatantAnimation::ApproachCombatant(target_id),
                ]))
            }
            CombatantVisualLocation::StandingInFrontOf(id) => {
                if target_id != id {
                    event_manager.animation_queue.append(&mut VecDeque::from([
                        CombatantAnimation::TurnToFaceCombatant(target_id),
                        CombatantAnimation::ApproachCombatant(target_id),
                    ]))
                }
            }
        }
        Ok(())
    })
}
