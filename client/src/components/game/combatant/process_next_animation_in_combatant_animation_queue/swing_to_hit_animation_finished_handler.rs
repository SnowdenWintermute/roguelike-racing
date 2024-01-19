use crate::components::mesh_manager::CombatantAnimation;
use crate::components::mesh_manager::FloatingNumber;
use crate::store::game_store::GameStore;
use common::app_consts::error_messages;
use common::errors::AppError;
use std::collections::VecDeque;
use yew::AttrValue;
use yewdux::Dispatch;

pub fn swing_to_hit_animation_finished_handler(
    game_dispatch: Dispatch<GameStore>,
    target_id: u32,
    hp_change_option: Option<i16>,
    evaded: bool,
    combatant_id: u32,
) -> Result<(), AppError> {
    game_dispatch.reduce_mut(|store| -> Result<(), AppError> {
        let game = store.get_current_game_mut()?;
        let (_, combatant_properties) = game.get_mut_combatant_by_id(&target_id)?;
        let new_hp = if let Some(hp_change) = hp_change_option {
            let new_hp = combatant_properties.change_hp(hp_change);
            store.combat_log.push(AttrValue::from(format!(
                "{combatant_id} hit {target_id} for {hp_change}"
            )));
            new_hp
        } else {
            combatant_properties.hit_points
        };

        let target_event_manager = store
            .action_results_manager
            .combantant_event_managers
            .get_mut(&target_id)
            .ok_or_else(|| AppError {
                error_type: common::errors::AppErrorTypes::ClientError,
                message: error_messages::COMBANTANT_EVENT_MANAGER_NOT_FOUND.to_string(),
            })?;

        if evaded {
            store.combat_log.push(AttrValue::from(format!(
                "{target_id} evaded an attack from {combatant_id}"
            )));
            if target_event_manager.action_result_queue.front().is_none() {
                target_event_manager.animation_queue = VecDeque::from([CombatantAnimation::Evasion])
            }
        }

        if let Some(hp_change) = hp_change_option {
            target_event_manager
                .floating_numbers_queue
                .push_back(FloatingNumber {
                    value: hp_change,
                    color: AttrValue::from("rgba(255,255,255,0)"),
                });
            // don't animate hit recovery if they hit themselves during an
            // action of their own doing
            if target_event_manager.action_result_queue.front().is_none() {
                if new_hp == 0 {
                    target_event_manager.animation_queue =
                        VecDeque::from([CombatantAnimation::Death(Some(hp_change))]);
                    store
                        .combat_log
                        .push(AttrValue::from(format!("{target_id} died")));
                } else {
                    target_event_manager.animation_queue =
                        VecDeque::from([CombatantAnimation::HitRecovery(hp_change)])
                }
            }
        }
        Ok(())
    })
}
