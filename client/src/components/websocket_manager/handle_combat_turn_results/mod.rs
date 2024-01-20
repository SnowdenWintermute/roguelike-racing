use crate::store::game_store::GameStore;
use common::app_consts::error_messages;
use common::combat::CombatTurnResult;
use common::errors::AppError;
use common::packets::server_to_client::BattleConclusion;
use yew::AttrValue;
use yewdux::Dispatch;

pub fn handle_combat_turn_results(
    game_dispatch: Dispatch<GameStore>,
    turn_results: Vec<CombatTurnResult>,
) -> Result<(), AppError> {
    game_dispatch.reduce_mut(|store| {
        for turn_result in turn_results {
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
            match battle_end_report.conclusion {
                BattleConclusion::Victory => {
                    let party = store.get_current_party_mut()?;
                    party.battle_id = None;
                    party.current_room.monsters = None;
                    if let Some(items) = &mut party.current_room.items {
                        if let Some(mut loot) = battle_end_report.loot {
                            items.append(&mut loot)
                        }
                    } else {
                        party.current_room.items = battle_end_report.loot
                    }
                    store
                        .combat_log
                        .push(AttrValue::from("battle ended in victory"))
                }
                BattleConclusion::Defeat => todo!(),
            }

            store.current_battle_end_report = None;
            store.current_battle_id = None;
        }
        Ok(())
    })
}
