use super::model_actions::CombatantModelActions;
use super::set_melee_target_destination_transform_and_rotation::set_melee_target_destination_transform_and_rotation;
use super::ModelActionQueue;
use super::TransformManager;
use crate::bevy_app::bevy_app_consts::COMBATANT_TIME_TO_TRAVEL_ONE_METER;
use crate::bevy_app::modular_character_plugin::spawn_combatant::ActionResultsProcessing;
use crate::bevy_app::modular_character_plugin::spawn_combatant::HitboxRadius;
use crate::bevy_app::modular_character_plugin::spawn_combatant::MainSkeletonEntity;
use crate::bevy_app::modular_character_plugin::CombatantsById;
use crate::bevy_app::modular_character_plugin::HomeLocation;
use crate::bevy_app::modular_character_plugin::TurnResultsQueue;
use crate::comm_channels::messages_from_bevy::MessageFromBevy;
use crate::comm_channels::BevyTransmitter;
use crate::comm_channels::ProcessNextTurnResultEvent;
use bevy::prelude::*;
use common::combat::combat_actions::CombatAction;
use common::combat::CombatTurnResult;
use common::combatants::abilities::CombatantAbilityNames;

pub fn process_next_turn_result_event_handler(
    mut proccess_next_turn_result_event_reader: EventReader<ProcessNextTurnResultEvent>,
    mut turn_results_queue: ResMut<TurnResultsQueue>,
    combatants_by_id: Res<CombatantsById>,
    mut combatants: Query<(
        &mut TransformManager,
        &mut ModelActionQueue,
        &mut ActionResultsProcessing,
        &MainSkeletonEntity,
        &HomeLocation,
    )>,
    target_combatants: Query<(&MainSkeletonEntity, &HitboxRadius)>,
    transforms: Query<&Transform>,
    bevy_transmitter: ResMut<BevyTransmitter>,
) {
    for event in proccess_next_turn_result_event_reader.read() {
        if let Some(combatant_id) = event.0 {
            let _result = bevy_transmitter
                .0
                .send(MessageFromBevy::FinishedProcessingTurnResult(combatant_id));
        }

        if let Some(CombatTurnResult {
            combatant_id,
            action_results,
        }) = turn_results_queue.0.pop_front()
        {
            let _result = bevy_transmitter
                .0
                .send(MessageFromBevy::StartedProcessingTurnResult(combatant_id));

            let combatant_entity = combatants_by_id
                .0
                .get(&combatant_id)
                .expect("to have registered the entity");

            let (
                mut transform_manager,
                mut model_action_queue,
                mut action_results_processing,
                skeleton_entity,
                home_location,
            ) = combatants
                .get_mut(*combatant_entity)
                .expect("to have the entity");

            // enqueue model actions from action result
            for (i, action_result) in action_results.into_iter().enumerate() {
                match &action_result.action {
                    CombatAction::AbilityUsed(ability_name) => {
                        // have set their destination so they have somewhere to "return home" from
                        if i == 0 {
                            // will set the destination below based on melee action or not
                            model_action_queue
                                .0
                                .push_back(CombatantModelActions::ApproachDestination);
                            if ability_name.get_attributes().is_melee {
                                set_melee_target_destination_transform_and_rotation(
                                    &action_result,
                                    &mut transform_manager,
                                    &combatants_by_id.0,
                                    skeleton_entity.0,
                                    &target_combatants,
                                    &transforms,
                                );
                            } else {
                                // if no destination was set, make it the home location
                                if transform_manager.destination.is_none() {
                                    let combatant_transform = transforms
                                        .get(skeleton_entity.0)
                                        .expect("to have the transformm")
                                        .clone();
                                    transform_manager.set_destination(
                                        combatant_transform.clone(),
                                        Some(home_location.0.clone()),
                                    );
                                }
                                if transform_manager.target_rotation.is_none() {
                                    transform_manager.target_rotation = None
                                }
                            }
                        }
                        let model_action = match ability_name {
                            // attack is only used by the client to show a generic menu option which is
                            // interpreted as one of the more specific attack types handled below
                            CombatantAbilityNames::Attack => {
                                CombatantModelActions::AttackMeleeMainHand
                            }
                            CombatantAbilityNames::AttackMeleeMainhand => {
                                CombatantModelActions::AttackMeleeMainHand
                            }
                            CombatantAbilityNames::AttackMeleeOffhand => {
                                CombatantModelActions::AttackMeleeOffHand
                            }
                            CombatantAbilityNames::AttackRangedMainhand => todo!(),
                            CombatantAbilityNames::Fire
                            | CombatantAbilityNames::Ice
                            | CombatantAbilityNames::Healing => CombatantModelActions::CastSpell,
                        };

                        model_action_queue.0.push_back(model_action)
                    }
                    CombatAction::ConsumableUsed(item) => {
                        //
                    }
                }
                // to be removed and read by relevant actions that need the damage/targets info etc
                action_results_processing.0.push(action_result)
            }

            model_action_queue
                .0
                .push_back(CombatantModelActions::ReturnHome);
            model_action_queue
                .0
                .push_back(CombatantModelActions::Recenter);
        } else {
            // no more turn results, tell yew to have the combatants take their next turn
        };
    }
}
