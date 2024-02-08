use crate::components::mesh_manager::CombatantAnimation;
use crate::store::game_store::GameStore;
use common::app_consts::error_messages;
use common::combat::ActionResult;
use common::errors::AppError;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use yewdux::Dispatch;

pub fn queue_fire_animations(
    game_dispatch: Dispatch<GameStore>,
    combatant_id: u32,
    action_result: &ActionResult,
) -> Result<(), AppError> {
    game_dispatch.reduce_mut(|store| -> Result<(), AppError> {
        let game = store.get_current_game()?;
        let party = store.get_current_party()?;
        let battle_id_option = party.battle_id;
        let current_battle_id = battle_id_option.ok_or_else(|| AppError {
            error_type: common::errors::AppErrorTypes::ClientError,
            message: error_messages::MISSING_BATTLE_REFERENCE.to_string(),
        })?;
        let battle = game
            .battles
            .get(&current_battle_id)
            .ok_or_else(|| AppError {
                error_type: common::errors::AppErrorTypes::ClientError,
                message: error_messages::BATTLE_NOT_FOUND.to_string(),
            })?;
        let (ally_ids, opponent_ids_option) = battle
            .get_ally_ids_and_opponent_ids_option(action_result.user_id)?
            .clone();
        let target_ids = action_result.targets.get_targets_if_scheme_valid(
            ally_ids,
            opponent_ids_option,
            vec![],
        )?;
        let hp_changes_by_entity_id = &action_result
            .hp_changes_by_entity_id
            .clone()
            .unwrap_or_else(|| HashMap::new());
        let evades_by_entity_id = &action_result
            .misses_by_entity_id
            .clone()
            .unwrap_or_else(|| HashSet::new());

        for target_id in target_ids {
            let hp_change_option = hp_changes_by_entity_id.get(&target_id);
            let evaded = evades_by_entity_id.get(&target_id).is_some();

            let event_manager = store
                .action_results_manager
                .combantant_event_managers
                .get_mut(&combatant_id)
                .expect("none checked");

            event_manager.animation_queue.append(&mut VecDeque::from([
                CombatantAnimation::SwingMainHandToHit(
                    target_id,
                    hp_change_option.copied(),
                    evaded,
                ),
                CombatantAnimation::MainHandFollowThroughSwing,
            ]));
        }

        Ok(())
    })
}
