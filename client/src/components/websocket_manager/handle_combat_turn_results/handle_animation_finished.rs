use super::process_next_event_in_turn_result_queue;
use crate::components::alerts::set_alert;
use crate::components::mesh_manager::ClientCombatantEvent;
use crate::components::websocket_manager::handle_combat_turn_results::add_event_to_combat_log::get_combat_log_entry;
use crate::store::alert_store::AlertStore;
use crate::store::game_store::get_current_battle_option;
use crate::store::game_store::GameStore;
use common::app_consts::error_messages;
use common::errors::AppError;
use common::utils::vec_shift;
use gloo::console::log;
use yewdux::Dispatch;

pub fn handle_event_finished_animating(
    associated_combatant_id: u32,
    combatant_event: ClientCombatantEvent,
    game_dispatch: Dispatch<GameStore>,
    alert_dispatch: Dispatch<AlertStore>,
) {
    let result = game_dispatch.reduce_mut(|store| -> Result<(), AppError> {
        let combatant_event_manager = store
            .action_results_manager
            .combantant_event_managers
            .get_mut(&associated_combatant_id)
            .ok_or_else(|| AppError {
                error_type: common::errors::AppErrorTypes::ClientError,
                message: error_messages::COMBANTANT_EVENT_MANAGER_NOT_FOUND.to_string(),
            })?;

        combatant_event_manager.current_event_processing = None;

        let battle_option = get_current_battle_option(store);
        let battle_id_option = if let Some(battle) = battle_option {
            Some(battle.id)
        } else {
            None
        };
        let party_id = store.current_party_id.ok_or_else(|| AppError {
            error_type: common::errors::AppErrorTypes::ClientError,
            message: error_messages::MISSING_PARTY_REFERENCE.to_string(),
        })?;
        let game = store.get_current_game()?;
        let combat_log_entry = get_combat_log_entry(
            game,
            associated_combatant_id,
            &combatant_event,
            party_id,
            battle_option,
        )?;

        match combatant_event {
            ClientCombatantEvent::TookAction(action_result) => {
                let ability_user_id = action_result.user_id;
                let game = store.get_current_game()?;
                let target_ids = game.get_ids_from_ability_target(
                    party_id,
                    battle_option,
                    &action_result.targets,
                    ability_user_id,
                )?;

                for entity_id in target_ids {
                    let cloned_alert_dispatch = alert_dispatch.clone();
                    let cloned_game_dispatch = game_dispatch.clone();
                    if let Some(hp_changes_by_id) = &action_result.hp_changes_by_entity_id {
                        if let Some(hp_change) = hp_changes_by_id.get(&entity_id) {
                            let game = store.get_current_game_mut()?;
                            let (_, combatant_properties) =
                                game.get_mut_combatant_by_id(&entity_id)?;
                            let new_hp = combatant_properties.change_hp(*hp_change);

                            // put the damage taken animation in this entity's event queue
                            let this_entity_event_manager = store
                                .action_results_manager
                                .combantant_event_managers
                                .get_mut(&entity_id)
                                .ok_or_else(|| AppError {
                                    error_type: common::errors::AppErrorTypes::ClientError,
                                    message: error_messages::COMBANTANT_EVENT_MANAGER_NOT_FOUND
                                        .to_string(),
                                })?;
                            if new_hp == 0 {
                                this_entity_event_manager.current_event_processing =
                                    Some(ClientCombatantEvent::Died);
                                    this_entity_event_manager.process_active_event(cloned_game_dispatch, cloned_alert_dispatch);
                            } else {
                                this_entity_event_manager.current_event_processing =
                                    Some(ClientCombatantEvent::HpChange(*hp_change));
                                    this_entity_event_manager.process_active_event(cloned_game_dispatch, cloned_alert_dispatch);
                            }
                        }
                    }
                }

                if action_result.ends_turn {
                    if let Some(battle_id) = battle_id_option {
                        let game = store.get_current_game_mut()?;
                        game.end_active_combatant_turn(battle_id)?;
                    }
                }

                if store.action_results_manager.turn_results_queue.len() > 0 {
                    log!(format!("calling process_next_event_in_turn_result_queue from handle_event_finished_animating, {:#?}",
                    store.action_results_manager.turn_results_queue));
                    let cloned_game_dispatch = game_dispatch.clone();
                    let cloned_alert_dispatch = alert_dispatch.clone();
                    let _ = vec_shift(&mut store.action_results_manager.turn_results_queue);
                    process_next_event_in_turn_result_queue(
                        cloned_game_dispatch,
                        cloned_alert_dispatch,
                    )?;
                }
            }
            _ => (),
        }

        store.combat_log.push(combat_log_entry);

        Ok(())
    });

    if let Some(err) = result.err() {
        set_alert(alert_dispatch, err.message.clone())
    }
}
