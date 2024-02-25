use std::collections::HashMap;

use crate::components::game::combat_log::combat_log_message::CombatLogMessage;
use crate::components::game::combat_log::combat_log_message::CombatLogMessageStyle;
use crate::store::game_store::GameStore;
use common::app_consts::error_messages;
use common::combat::CombatTurnResult;
use common::combatants::award_levelups::award_levelups;
use common::errors::AppError;
use common::packets::server_to_client::BattleConclusion;
use gloo::console::log;
use yew::AttrValue;
use yewdux::Dispatch;

pub fn handle_combat_turn_results(
    game_dispatch: Dispatch<GameStore>,
    turn_results: Vec<CombatTurnResult>,
) -> Result<(), AppError> {
    game_dispatch.reduce_mut(|store| {
        for turn_result in turn_results {
            log!(format!("turn result: {:#?}", turn_result));
            store
                .action_results_manager
                .turn_results_queue
                .push_back(turn_result)
        }
    });
    send_next_turn_result_to_combatant_event_manager(game_dispatch.clone())
}

pub fn send_next_turn_result_to_combatant_event_manager(
    game_dispatch: Dispatch<GameStore>,
) -> Result<(), AppError> {
    game_dispatch.reduce_mut(|store| -> Result<(), AppError> {
        let next_turn_to_process_option =
            store.action_results_manager.turn_results_queue.pop_front();
        if let Some(next_turn_to_process) = next_turn_to_process_option {
            for action_result in next_turn_to_process.action_results {
                store
                    .action_results_manager
                    .combantant_event_managers
                    .get_mut(&next_turn_to_process.combatant_id)
                    .ok_or_else(|| AppError {
                        error_type: common::errors::AppErrorTypes::ClientError,
                        message: error_messages::COMBANTANT_EVENT_MANAGER_NOT_FOUND.to_string(),
                    })?
                    .action_result_queue
                    .push_back(action_result);
            }
        } else if let Some(battle_end_report) = store.current_battle_end_report.clone() {
            let party = store.get_current_party_mut()?;
            match battle_end_report.conclusion {
                BattleConclusion::Victory => {
                    party.current_room.monsters = None;
                    party.battle_id = None;
                    store.combat_log.push(CombatLogMessage::new(
                        AttrValue::from("battle ended in victory"),
                        CombatLogMessageStyle::BattleVictory,
                        0,
                    ));
                    let mut entity_ids_and_previous_levels = HashMap::new();
                    // HANDLE LEVELUPS
                    if let Some(exp_changes) = battle_end_report.exp_changes {
                        for exp_change in exp_changes {
                            let entity_id = exp_change.combatant_id;
                            let party = store.get_current_party_mut()?;
                            let (entity_properties, combatant_properties) =
                                party.get_mut_combatant_by_id(&exp_change.combatant_id)?;
                            let entity_name = entity_properties.name.clone();
                            entity_ids_and_previous_levels
                                .insert(entity_id, combatant_properties.level);
                            if exp_change.experience_change > 0 {
                                combatant_properties.experience_points.current +=
                                    exp_change.experience_change.abs() as u16;
                            } else {
                                combatant_properties.experience_points.current -=
                                    exp_change.experience_change.abs() as u16;
                            }
                            award_levelups(combatant_properties);

                            store.combat_log.push(CombatLogMessage::new(
                                AttrValue::from(format!(
                                    "{} gained {} experience points",
                                    entity_name, exp_change.experience_change,
                                )),
                                CombatLogMessageStyle::PartyProgress,
                                0,
                            ));
                        }

                        for (id, level) in &entity_ids_and_previous_levels {
                            let party = store.get_current_party_mut()?;
                            let (entity_properties, combatant_properties) =
                                party.get_mut_combatant_by_id(&id)?;
                            let name = entity_properties.name.clone();
                            let new_level = combatant_properties.level;
                            if *level != new_level {
                                store.combat_log.push(CombatLogMessage::new(
                                    AttrValue::from(format!("{} is now level {}", name, new_level)),
                                    CombatLogMessageStyle::PartyProgress,
                                    0,
                                ));
                            }
                        }
                    }

                    store.current_battle_id = None;
                }
                BattleConclusion::Defeat => {
                    party.time_of_wipe = Some(js_sys::Date::now() as u64);
                    store.combat_log.push(CombatLogMessage::new(
                        AttrValue::from("battle ended in defeat"),
                        CombatLogMessageStyle::PartyWipe,
                        0,
                    ));
                }
            }

            store.current_battle_end_report = None;
        }
        Ok(())
    })
}
