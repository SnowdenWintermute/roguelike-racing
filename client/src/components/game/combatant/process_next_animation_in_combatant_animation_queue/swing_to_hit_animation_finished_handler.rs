use crate::components::game::combat_log::combat_log_message::CombatLogMessage;
use crate::components::game::combat_log::combat_log_message::CombatLogMessageStyle;
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
        let party = store.get_current_party_mut()?;
        let battle_id_option = party.battle_id;
        let game = store.get_current_game_mut()?;
        let (attacker_entity_properties, _) = game.get_mut_combatant_by_id(&combatant_id)?;
        let attacker_name = attacker_entity_properties.name.clone();
        let (entity_properties, combatant_properties) = game.get_mut_combatant_by_id(&target_id)?;
        let entity_id = entity_properties.id;
        let target_name = entity_properties.name.clone();
        let new_hp = if let Some(hp_change) = hp_change_option {
            let new_hp = combatant_properties.change_hp(hp_change);
            store.combat_log.push(CombatLogMessage::new(
                AttrValue::from(format!(
                    "{} ({combatant_id}) hit {} ({target_id}) for {hp_change}",
                    attacker_name, target_name
                )),
                CombatLogMessageStyle::Basic,
                0,
            ));
            new_hp
        } else {
            combatant_properties.hit_points
        };

        // REMOVE THEIR TURN TRACKER
        if new_hp == 0 {
            let game = store.get_current_game_mut()?;
            if let Some(battle_id) = battle_id_option {
                let battle = game.battles.get_mut(&battle_id).ok_or_else(|| AppError {
                    error_type: common::errors::AppErrorTypes::ClientError,
                    message: error_messages::BATTLE_NOT_FOUND.to_string(),
                })?;
                let mut index_to_remove_option = None;
                for (i, turn_tracker) in battle.combatant_turn_trackers.iter().enumerate() {
                    if turn_tracker.entity_id == entity_id {
                        index_to_remove_option = Some(i)
                    }
                }
                if let Some(index_to_remove) = index_to_remove_option {
                    let _ = battle.combatant_turn_trackers.remove(index_to_remove);
                }
            }
        }

        let target_event_manager = store
            .action_results_manager
            .combantant_event_managers
            .get_mut(&target_id)
            .ok_or_else(|| AppError {
                error_type: common::errors::AppErrorTypes::ClientError,
                message: error_messages::COMBANTANT_EVENT_MANAGER_NOT_FOUND.to_string(),
            })?;

        if evaded {
            store.combat_log.push(CombatLogMessage::new(
                AttrValue::from(format!(
                    "{} ({target_id}) evaded an attack from {} ({combatant_id})",
                    target_name, attacker_name,
                )),
                CombatLogMessageStyle::Basic,
                0,
            ));
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

            if target_event_manager.action_result_queue.front().is_none() {
                if new_hp == 0 {
                    target_event_manager.animation_queue =
                        VecDeque::from([CombatantAnimation::Death(Some(hp_change))]);
                    store.combat_log.push(CombatLogMessage::new(
                        AttrValue::from(format!("{} ({target_id}) died", target_name)),
                        CombatLogMessageStyle::Basic,
                        0,
                    ));
                } else if combatant_id != target_id {
                    // don't hit recovery if attacking self or else return to home animation won't
                    // play and trigger next
                    target_event_manager.animation_queue =
                        VecDeque::from([CombatantAnimation::HitRecovery(hp_change)])
                }
            }
        }
        Ok(())
    })
}
