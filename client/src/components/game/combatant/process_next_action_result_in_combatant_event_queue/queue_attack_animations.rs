use crate::components::mesh_manager::CombatantAnimation;
use crate::store::game_store::GameStore;
use common::app_consts::error_messages;
use common::combat::combat_actions::CombatActionTarget;
use common::combat::ActionResult;
use common::errors::AppError;
use std::collections::VecDeque;
use yewdux::Dispatch;

pub fn queue_attack_animations(
    game_dispatch: Dispatch<GameStore>,
    combatant_id: u32,
    action_result: &ActionResult,
) -> Result<(), AppError> {
    game_dispatch.reduce_mut(|store| -> Result<(), AppError> {
        let event_manager = store
            .action_results_manager
            .combantant_event_managers
            .get_mut(&combatant_id)
            .expect("none checked");

        let target_id = match action_result.targets {
            CombatActionTarget::Single(id) => id,
            _ => {
                return Err(AppError {
                    error_type: common::errors::AppErrorTypes::Generic,
                    message: error_messages::INVALID_TARGETING_SCHEME.to_string(),
                })
            }
        };

        let hp_change_option =
            if let Some(hp_changes_by_entity) = &action_result.hp_changes_by_entity_id {
                hp_changes_by_entity.get(&target_id)
            } else {
                None
            };
        let evaded = if let Some(misses_by_entity_id) = &action_result.misses_by_entity_id {
            misses_by_entity_id.contains(&target_id)
        } else {
            false
        };

        event_manager.animation_queue.append(&mut VecDeque::from([
            CombatantAnimation::SwingMainHandToHit(target_id, hp_change_option.copied(), evaded),
            CombatantAnimation::MainHandFollowThroughSwing,
        ]));

        Ok(())
    })
}
