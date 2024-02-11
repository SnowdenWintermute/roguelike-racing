use crate::components::mesh_manager::CombatantAnimation;
use crate::components::mesh_manager::HpChange;
use crate::components::mesh_manager::HpChangeResult;
use crate::components::mesh_manager::TargetAndHpChangeResults;
use crate::store::game_store::GameStore;
use common::app_consts::error_messages;
use common::combat::combat_actions::filter_possible_target_ids_by_prohibited_combatant_states::filter_possible_target_ids_by_prohibited_combatant_states;
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
        let combat_action_properties = action_result
            .action
            .get_properties_if_owned(game, combatant_id)?;
        let (filtered_ally_ids, filtered_opponent_ids_option) =
            filter_possible_target_ids_by_prohibited_combatant_states(
                game,
                &combat_action_properties.prohibited_target_combatant_states,
                ally_ids.clone(),
                opponent_ids_option.clone(),
            )?;

        let target_ids = action_result.targets.get_targets_if_scheme_valid(
            filtered_ally_ids,
            filtered_opponent_ids_option,
            vec![],
        )?;
        let hp_changes_by_entity_id = &action_result
            .hp_changes_by_entity_id
            .clone()
            .unwrap_or_else(|| HashMap::new());
        let crits_by_entity_id = &action_result
            .crits_by_entity_id
            .clone()
            .unwrap_or_else(|| HashSet::new());
        let evades_by_entity_id = &action_result
            .misses_by_entity_id
            .clone()
            .unwrap_or_else(|| HashSet::new());
        let mp_combat_action_prices_paid_by_entity_id = &action_result
            .mp_combat_action_prices_paid_by_entity_id
            .clone()
            .unwrap_or_else(|| HashMap::new());
        let mp_price = mp_combat_action_prices_paid_by_entity_id
            .get(&combatant_id)
            .unwrap_or_else(|| &0);

        let mut hp_change_results = vec![];

        for target_id in target_ids {
            let evaded = evades_by_entity_id.get(&target_id).is_some();
            let hp_change_result = if evaded {
                HpChangeResult::Evaded
            } else {
                let hp_change = hp_changes_by_entity_id
                    .get(&target_id)
                    .expect("to have an hp_change_option");
                let is_crit = crits_by_entity_id.contains(&target_id);
                if hp_change <= &0 {
                    HpChangeResult::Damaged(HpChange {
                        value: *hp_change,
                        is_crit,
                    })
                } else {
                    HpChangeResult::Healed(HpChange {
                        value: *hp_change,
                        is_crit,
                    })
                }
            };
            hp_change_results.push(TargetAndHpChangeResults {
                target_id,
                hp_change_result,
            })
        }
        let event_manager = store
            .action_results_manager
            .combantant_event_managers
            .get_mut(&combatant_id)
            .expect("none checked");

        event_manager.animation_queue.append(&mut VecDeque::from([
            CombatantAnimation::MoveForwardToCastSpell(*mp_price),
            CombatantAnimation::CastSpellOnTargets(hp_change_results),
            CombatantAnimation::MainHandFollowThroughSwing,
        ]));

        Ok(())
    })
}
