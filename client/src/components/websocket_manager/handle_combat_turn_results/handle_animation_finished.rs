use super::process_next_event_in_turn_result_queue;
use crate::components::alerts::set_alert;
use crate::components::mesh_manager::ClientCombatantEvent;
use crate::store::alert_store::AlertStore;
use crate::store::game_store::get_current_battle_option;
use crate::store::game_store::GameStore;
use common::app_consts::error_messages;
use common::errors::AppError;
use gloo::console::log;
use yewdux::Dispatch;

pub fn handle_event_finished_animating(
    associated_combatant_id: u32,
    combatant_event: ClientCombatantEvent,
    game_dispatch: Dispatch<GameStore>,
    alert_dispatch: Dispatch<AlertStore>,
) {
    let result = game_dispatch.reduce_mut(|store| -> Result<(), AppError> {
        // - queue and start damage taken animations on affected entities
        // - subtract hp from affected entities
        // - if any affected entity is dead, queue death animation on that entity
        // - if action required turn, end active combatant turn for the current battle if any
        match combatant_event {
            ClientCombatantEvent::TookAction(action_result) => {
                let battle_option = get_current_battle_option(store);
                let battle_id_option = if let Some(battle) = battle_option {
                    Some(battle.id)
                } else {
                    None
                };
                let ability_user_id = action_result.user_id;
                let party_id = store.current_party_id.ok_or_else(|| AppError {
                    error_type: common::errors::AppErrorTypes::ClientError,
                    message: error_messages::MISSING_PARTY_REFERENCE.to_string(),
                })?;
                let game = store.get_current_game()?;
                let target_ids = game.get_ids_from_ability_target(
                    party_id,
                    battle_option,
                    &action_result.targets,
                    ability_user_id,
                )?;
                log!(format!("target ids: {:#?}", target_ids));

                for entity_id in target_ids {
                    let cloned_game_dispatch = game_dispatch.clone();
                    let cloned_alert_dispatch = alert_dispatch.clone();
                    if let Some(hp_changes_by_id) = &action_result.hp_changes_by_entity_id {
                        log!(format!("hp changes by id: {:#?}", hp_changes_by_id));
                        if let Some(hp_change) = hp_changes_by_id.get(&entity_id) {
                            let game = store.get_current_game_mut()?;
                            let (_, combatant_properties) =
                                game.get_mut_combatant_by_id(&entity_id)?;
                            let new_hp = combatant_properties.change_hp(*hp_change);

                            log!(format!("new hp for entity id {entity_id}: {:#?}", new_hp));
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
                            this_entity_event_manager
                                .event_queue
                                .push(ClientCombatantEvent::HpChange(*hp_change));
                            if new_hp == 0 {
                                this_entity_event_manager
                                    .event_queue
                                    .push(ClientCombatantEvent::Died);
                            }
                            // start processing their event queue if not already doing so
                            this_entity_event_manager
                                .process_next_event(cloned_game_dispatch, cloned_alert_dispatch);
                        }
                    }
                }

                let action_required_turn = match action_result.action {
                    common::combat::CombatAction::AbilityUsed(ability_name) => {
                        ability_name.get_attributes().requires_combat_turn
                    }
                    common::combat::CombatAction::ItemUsed(_) => false,
                };

                if action_required_turn {
                    if let Some(battle_id) = battle_id_option {
                        let game = store.get_current_game_mut()?;
                        game.end_active_combatant_turn(battle_id)?;
                    }
                }
            }
            _ => (),
        }

        // for any event animation finishing in the associated combatant's queue
        //  - process next event in that entity's queue
        //  - if all entity event queues are empty and no animations are ongoing,
        //    and in combat
        //    query the ActionResultsManager turn_results_queue queue for the next action_result to process/animate
        let event_manager_option = store
            .action_results_manager
            .combantant_event_managers
            .get_mut(&associated_combatant_id);
        let cloned_game_dispatch = game_dispatch.clone();
        let cloned_alert_dispatch = alert_dispatch.clone();
        if let Some(event_manager) = event_manager_option {
            event_manager.process_next_event(cloned_game_dispatch, cloned_alert_dispatch);
        }

        let should_process_next_turn_result_in_queue =
            if let Some(battle) = get_current_battle_option(store) {
                let all_combatant_ids_in_battle = battle.get_all_combatant_ids();
                let mut all_combatant_event_managers_done_processing = true;
                for combatant_id in all_combatant_ids_in_battle {
                    let this_entity_event_manager = store
                        .action_results_manager
                        .combantant_event_managers
                        .get_mut(&combatant_id)
                        .ok_or_else(|| AppError {
                            error_type: common::errors::AppErrorTypes::ClientError,
                            message: error_messages::COMBANTANT_EVENT_MANAGER_NOT_FOUND.to_string(),
                        })?;
                    if this_entity_event_manager.current_event_processing.is_some() {
                        all_combatant_event_managers_done_processing = false;
                        break;
                    }
                }
                all_combatant_event_managers_done_processing
            } else {
                false
            };

        let cloned_game_dispatch = game_dispatch.clone();
        let cloned_alert_dispatch = alert_dispatch.clone();
        if should_process_next_turn_result_in_queue {
            process_next_event_in_turn_result_queue(cloned_game_dispatch, cloned_alert_dispatch)?;
        }

        Ok(())
    });

    if let Some(err) = result.err() {
        set_alert(alert_dispatch, err.message)
    }
}
