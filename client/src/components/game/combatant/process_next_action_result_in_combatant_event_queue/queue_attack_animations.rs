use crate::components::mesh_manager::CombatantAnimation;
use crate::components::mesh_manager::HpChange;
use crate::components::mesh_manager::HpChangeResult;
use crate::components::mesh_manager::TargetAndHpChangeResults;
use crate::store::game_store::GameStore;
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

        let target_id = action_result.targets.get_single_target_id()?;
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
        let hp_change_value = hp_change_option.unwrap_or_else(|| &0);

        let is_crit = if let Some(crits_by_id) = &action_result.crits_by_entity_id {
            crits_by_id.get(&target_id).is_some()
        } else {
            false
        };

        let hp_change_result = if evaded {
            HpChangeResult::Evaded
        } else {
            HpChangeResult::Damaged(HpChange {
                value: *hp_change_value,
                is_crit,
            })
        };

        event_manager.animation_queue.append(&mut VecDeque::from([
            CombatantAnimation::SwingMainHandToHit(vec![TargetAndHpChangeResults {
                target_id,
                hp_change_result,
                combat_action: action_result.action.clone(),
            }]),
            CombatantAnimation::MainHandFollowThroughSwing,
        ]));

        Ok(())
    })
}
