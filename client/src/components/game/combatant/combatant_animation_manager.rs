use crate::components::mesh_manager::CombatantAnimation;
use crate::components::mesh_manager::FloatingNumber;
use crate::store::game_store::GameStore;
use common::app_consts::error_messages;
use common::combat::CombatAction;
use common::combatants::abilities::AbilityTarget;
use common::combatants::abilities::CombatantAbilityNames;
use common::errors::AppError;
use gloo::console::log;
use std::collections::VecDeque;
use yew::prelude::*;
use yewdux::use_store;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub combatant_id: u32,
}

#[function_component(Combatant)]
pub fn combatant_animation_manager(props: &Props) -> Html {
    // let (game_state, game_dispatch) = use_store::<GameStore>();
    // let timer_state = use_state(|| None);
    // let Props { combatant_id } = props;
    // let combatant_id = combatant_id.clone();
    // let event_manager_option = game_state
    //     .action_results_manager
    //     .combantant_event_managers
    //     .get(&combatant_id);
    // if event_manager_option.is_none() {
    //     return html!({ error_messages::COMBANTANT_EVENT_MANAGER_NOT_FOUND });
    // }
    // let event_manager = event_manager_option.expect("none checked");
    // let current_action_processing = event_manager.action_result_queue.front();

    // let cloned_game_dispatch = game_dispatch.clone();
    // use_effect_with(
    //     current_action_processing,
    //     move |current_action_processing| {
    //         if let Some(new_action_result) = &current_action_processing {
    //             //   current action result processing (if any)
    //             //   when an action result is passed, start animating
    //             //    - push to a queue of animations (move, swing to hit [damage here], follow through, recover, return)
    //             //    - animations have an on_finish which can trigger animations on other entities, interrupting
    //             //    their current hit recovery animation if any (getting hit before hit recovery animation finishes). Trigger
    //             //    the on_finish for that animation (floating numbers, combat log entry)
    //             //    - if take damage while in an action animation, just reduce the hp and show the floating
    //             //    numbers
    //             //    - if die while in an action animation, show floating numbers and, play the death animation in place
    //             //    - entities can not select (or execute) a new action until their animaton queues are finished
    //             let result = match &new_action_result.action {
    //                 CombatAction::AbilityUsed(ability_name) => match ability_name {
    //                     CombatantAbilityNames::Attack => {
    //                         cloned_game_dispatch.reduce_mut(|store| -> Result<(), AppError> {
    //                             let event_manager = store
    //                                 .action_results_manager
    //                                 .combantant_event_managers
    //                                 .get_mut(&combatant_id)
    //                                 .expect("none checked");

    //                             let target_id = match new_action_result.targets {
    //                                 AbilityTarget::Single(id) => id,
    //                                 _ => {
    //                                     return Err(AppError {
    //                                         error_type: common::errors::AppErrorTypes::Generic,
    //                                         message: error_messages::INVALID_TARGETING_SCHEME
    //                                             .to_string(),
    //                                     })
    //                                 }
    //                             };

    //                             let hp_change_option = if let Some(hp_changes_by_entity) =
    //                                 &new_action_result.hp_changes_by_entity_id
    //                             {
    //                                 hp_changes_by_entity.get(&target_id)
    //                             } else {
    //                                 None
    //                             };

    //                             event_manager.animation_queue = VecDeque::from([
    //                                 CombatantAnimation::TurnToFaceCombatant(target_id),
    //                                 CombatantAnimation::ApproachCombatant(target_id),
    //                                 CombatantAnimation::SwingMainHandToHit(
    //                                     target_id,
    //                                     hp_change_option.copied(),
    //                                 ),
    //                                 CombatantAnimation::MainHandFollowThroughSwing,
    //                             ]);

    //                             Ok(())
    //                         })
    //                     }
    //                     _ => Ok(()),
    //                 },
    //                 CombatAction::ItemUsed(_) => todo!(),
    //             };
    //         }
    //     },
    // );

    // let cloned_animation_queue = event_manager.animation_queue.clone();
    // let cloned_game_dispatch = game_dispatch.clone();
    // use_effect_with(cloned_animation_queue, move |cloned_animation_queue| {
    //     if let Some(animation) = cloned_animation_queue.front() {
    //         let cloned_animation = animation.clone();
    //         timer_state.set(Some(gloo::timers::callback::Timeout::new(
    //             1000,
    //             move || {
    //                 let result = match cloned_animation {
    //                     CombatantAnimation::TurnToFaceCombatant(target_id) => {
    //                         log!(format!("{} turned to face {}", combatant_id, target_id));
    //                         Ok(())
    //                     }
    //                     CombatantAnimation::ApproachCombatant(target_id) => {
    //                         log!(format!("{} approached {}", combatant_id, target_id));
    //                         Ok(())
    //                     }
    //                     CombatantAnimation::SwingMainHandToHit(target_id, hp_change_option) => {
    //                         cloned_game_dispatch.reduce_mut(|store| -> Result<(), AppError> {
    //                             let game = store.get_current_game_mut()?;
    //                             let (entity_properties, combatant_properties) =
    //                                 game.get_mut_combatant_by_id(&target_id)?;
    //                             let new_hp = if let Some(hp_change) = hp_change_option {
    //                                 let new_hp = combatant_properties.change_hp(hp_change);
    //                                 store.combat_log.push(AttrValue::from(format!(
    //                                     "{combatant_id} hit {target_id} for {hp_change}"
    //                                 )));
    //                                 new_hp
    //                             } else {
    //                                 combatant_properties.hit_points
    //                             };
    //                             let target_event_manager = store
    //                                 .action_results_manager
    //                                 .combantant_event_managers
    //                                 .get_mut(&target_id)
    //                                 .ok_or_else(|| AppError {
    //                                     error_type: common::errors::AppErrorTypes::ClientError,
    //                                     message: error_messages::COMBANTANT_EVENT_MANAGER_NOT_FOUND
    //                                         .to_string(),
    //                                 })?;
    //                             if let Some(hp_change) = hp_change_option {
    //                                 target_event_manager.floating_numbers_queue.push_back(
    //                                     FloatingNumber {
    //                                         value: hp_change,
    //                                         color: AttrValue::from("rgba(255,255,255,0)"),
    //                                     },
    //                                 );
    //                                 // don't animate hit recovery if they hit themselves during an
    //                                 // action of their own doing
    //                                 if target_event_manager.action_result_queue.front().is_none() {
    //                                     if new_hp == 0 {
    //                                         target_event_manager.animation_queue =
    //                                             VecDeque::from([CombatantAnimation::Death(Some(
    //                                                 hp_change,
    //                                             ))])
    //                                     } else {
    //                                         target_event_manager.animation_queue =
    //                                             VecDeque::from([CombatantAnimation::HitRecovery(
    //                                                 hp_change,
    //                                             )])
    //                                     }
    //                                 }
    //                             }
    //                             Ok(())
    //                         })
    //                     }
    //                     CombatantAnimation::SwingOffHandToHit => todo!(),
    //                     CombatantAnimation::MainHandFollowThroughSwing => todo!(),
    //                     CombatantAnimation::OffHandFollowThroughSwing => todo!(),
    //                     CombatantAnimation::ReturnToReadyPosition => todo!(),
    //                     CombatantAnimation::HitRecovery(_) => todo!(),
    //                     CombatantAnimation::Death(_) => todo!(),
    //                 };

    //                 let result = cloned_game_dispatch.reduce_mut(|store| -> Result<(), AppError> {
    //                     let event_manager = store
    //                         .action_results_manager
    //                         .combantant_event_managers
    //                         .get_mut(&combatant_id)
    //                         .ok_or_else(|| AppError {
    //                             error_type: common::errors::AppErrorTypes::ClientError,
    //                             message: error_messages::COMBANTANT_EVENT_MANAGER_NOT_FOUND
    //                                 .to_string(),
    //                         })?;
    //                     let finished_animation = event_manager.animation_queue.remove(0);

    //                     // match finished_animation {
    //                     //     CombatantAnimation::SwingMainHandToHit(_, _) => {
    //                     //         todo!()
    //                     //     }
    //                     //     CombatantAnimation::SwingOffHandToHit => todo!(),
    //                     //     _ => (),
    //                     // }
    //                     Ok(())
    //                 });
    //             },
    //         )));
    //     }
    // });

    html!()
}
