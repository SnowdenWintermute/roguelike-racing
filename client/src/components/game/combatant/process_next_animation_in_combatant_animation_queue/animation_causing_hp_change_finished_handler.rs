use crate::components::game::combat_log::combat_log_message::CombatLogMessage;
use crate::components::game::combat_log::combat_log_message::CombatLogMessageStyle;
use crate::components::mesh_manager::CombatantAnimation;
use crate::components::mesh_manager::HpChange;
use crate::components::mesh_manager::HpChangeResult;
use crate::components::mesh_manager::TargetAndHpChangeResults;
use crate::store::game_store::GameStore;
use common::app_consts::error_messages;
use common::combat::combat_actions::CombatAction;
use common::combatants::abilities::CombatantAbilityNames;
use common::errors::AppError;
use common::game::RoguelikeRacerGame;
use std::collections::VecDeque;
use yew::AttrValue;
use yewdux::Dispatch;

pub fn animation_causing_hp_change_finished_handler(
    game_dispatch: Dispatch<GameStore>,
    targets_and_hp_change_results: Vec<TargetAndHpChangeResults>,
    causer_id: u32,
) -> Result<(), AppError> {
    game_dispatch.reduce_mut(|store| -> Result<(), AppError> {
        let party = store.get_current_party_mut()?;
        let battle_id_option = party.battle_id;
        let game = store.get_current_game_mut()?;
        let (causer_entity_properties, _) = game.get_mut_combatant_by_id(&causer_id)?;
        let causer_name = causer_entity_properties.name.clone();

        let combat_log_ability_use_option =
            if let Some(target_and_result) = targets_and_hp_change_results.first() {
                match &target_and_result.combat_action {
                    CombatAction::AbilityUsed(ability_name) => match ability_name {
                        CombatantAbilityNames::Attack => None,
                        CombatantAbilityNames::AttackMeleeMainhand => None,
                        CombatantAbilityNames::AttackMeleeOffhand => None,
                        CombatantAbilityNames::AttackRangedMainhand => None,
                        CombatantAbilityNames::Fire
                        | CombatantAbilityNames::Healing
                        | CombatantAbilityNames::Ice => {
                            Some(format!("{causer_name} casts {ability_name}"))
                        }
                    },
                    CombatAction::ConsumableUsed(_) => None,
                }
            } else {
                None
            };

        if let Some(ability_use_message) = combat_log_ability_use_option {
            store.combat_log.push(CombatLogMessage::new(
                AttrValue::from(ability_use_message),
                CombatLogMessageStyle::Basic,
                0,
            ));
        }

        for target_and_hp_change_result in targets_and_hp_change_results {
            let TargetAndHpChangeResults {
                target_id,
                hp_change_result,
                combat_action,
            } = target_and_hp_change_result;
            let game = store.get_current_game_mut()?;
            let (entity_properties, _) = game.get_mut_combatant_by_id(&target_id)?;
            let target_name = entity_properties.name.clone();

            let target_event_manager = store
                .action_results_manager
                .combantant_event_managers
                .get_mut(&target_id)
                .ok_or_else(|| AppError {
                    error_type: common::errors::AppErrorTypes::ClientError,
                    message: error_messages::COMBANTANT_EVENT_MANAGER_NOT_FOUND.to_string(),
                })?;

            match hp_change_result {
                HpChangeResult::Evaded => {
                    store.combat_log.push(CombatLogMessage::new(
                        AttrValue::from(format!(
                            "{} missed their attack on {}",
                            causer_name, target_name
                        )),
                        CombatLogMessageStyle::Basic,
                        0,
                    ));
                    if target_event_manager.action_result_queue.front().is_none() {
                        target_event_manager.animation_queue =
                            VecDeque::from([CombatantAnimation::Evasion])
                    }
                }
                HpChangeResult::Damaged(HpChange { value, is_crit }) => {
                    match is_crit {
                        true => {
                            store.combat_log.push(CombatLogMessage::new(
                                AttrValue::from(format!("{} scores a critical hit!", causer_name,)),
                                CombatLogMessageStyle::Basic,
                                0,
                            ));
                            store.combat_log.push(CombatLogMessage::new(
                                AttrValue::from(format!(
                                    "{} takes {} points of damage.",
                                    target_name,
                                    value * -1
                                )),
                                CombatLogMessageStyle::Basic,
                                0,
                            ));
                        }
                        false => {
                            store.combat_log.push(CombatLogMessage::new(
                                AttrValue::from(format!(
                                    "{} hits {} for {} points of damage.",
                                    causer_name,
                                    target_name,
                                    value * -1
                                )),
                                CombatLogMessageStyle::Basic,
                                0,
                            ));
                        }
                    }

                    let game = store.get_current_game_mut()?;
                    let (_, combatant_properties) = game.get_mut_combatant_by_id(&target_id)?;
                    let new_hp = combatant_properties.change_hp(value);

                    if new_hp == 0 {
                        remove_combatant_turn_tracker(game, battle_id_option, target_id)?;
                        let target_event_manager = store
                            .action_results_manager
                            .combantant_event_managers
                            .get_mut(&target_id)
                            .ok_or_else(|| AppError {
                                error_type: common::errors::AppErrorTypes::ClientError,
                                message: error_messages::COMBANTANT_EVENT_MANAGER_NOT_FOUND
                                    .to_string(),
                            })?;

                        target_event_manager
                            .animation_queue
                            .push_back(CombatantAnimation::Death(Some(value)));
                        store.combat_log.push(CombatLogMessage::new(
                            AttrValue::from(format!(
                                "{} reduced {}'s HP to zero.",
                                causer_name, target_name
                            )),
                            CombatLogMessageStyle::Basic,
                            0,
                        ));
                    }

                    if new_hp != 0 && causer_id != target_id {
                        // don't hit recovery if attacking self or else return to home animation won't
                        // play and trigger next
                        let target_event_manager = store
                            .action_results_manager
                            .combantant_event_managers
                            .get_mut(&target_id)
                            .ok_or_else(|| AppError {
                                error_type: common::errors::AppErrorTypes::ClientError,
                                message: error_messages::COMBANTANT_EVENT_MANAGER_NOT_FOUND
                                    .to_string(),
                            })?;

                        target_event_manager.animation_queue =
                            VecDeque::from([CombatantAnimation::HitRecovery(value)])
                    }

                    // target_event_manager
                    //     .floating_numbers_queue
                    //     .push_back(FloatingNumber {
                    //         value: hp_change,
                    //         color: AttrValue::from("rgba(255,255,255,0)"),
                    //     });
                }
                HpChangeResult::Healed(HpChange { value, is_crit: _ }) => {
                    store.combat_log.push(CombatLogMessage::new(
                        AttrValue::from(format!(
                            "{} healed {} for {} HP.",
                            causer_name, target_name, value
                        )),
                        CombatLogMessageStyle::Basic,
                        0,
                    ));

                    let game = store.get_current_game_mut()?;
                    let (_, combatant_properties) = game.get_mut_combatant_by_id(&target_id)?;
                    let _ = combatant_properties.change_hp(value);

                    // target_event_manager
                    //     .floating_numbers_queue
                    //     .push_back(FloatingNumber {
                    //         value: hp_change,
                    //         color: AttrValue::from("rgba(255,255,255,0)"),
                    //     });
                }
            }
        }
        Ok(())
    })
}

fn remove_combatant_turn_tracker(
    game: &mut RoguelikeRacerGame,
    battle_id_option: Option<u32>,
    entity_id: u32,
) -> Result<(), AppError> {
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
    Ok(())
}
