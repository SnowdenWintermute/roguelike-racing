use super::set_melee_target_destination_transform_and_rotation::set_melee_target_destination_transform_and_rotation;
use super::set_non_melee_ability_destination_transform_and_rotation::set_non_melee_ability_destination_transform_and_rotation;
use super::ModelActionQueue;
use super::TransformManager;
use crate::bevy_app::modular_character_plugin::process_combatant_model_actions::model_actions::CombatantModelActions;
use crate::bevy_app::modular_character_plugin::spawn_combatant::ActionResultsProcessing;
use crate::bevy_app::modular_character_plugin::spawn_combatant::HitboxRadius;
use crate::bevy_app::modular_character_plugin::spawn_combatant::MainSkeletonEntity;
use crate::bevy_app::modular_character_plugin::CombatantsById;
use crate::bevy_app::modular_character_plugin::RawActionResultsQueue;
use crate::comm_channels::messages_from_bevy::MessageFromBevy;
use crate::comm_channels::BevyTransmitter;
use bevy::prelude::*;
use common::combat::combat_actions::CombatAction;
use common::combatants::abilities::CombatantAbilityNames;

pub fn process_new_raw_action_results_handler(
    mut new_action_results: ResMut<RawActionResultsQueue>,
    combatants_by_id: Res<CombatantsById>,
    mut combatants: Query<(
        &mut TransformManager,
        &mut ModelActionQueue,
        &mut ActionResultsProcessing,
        &MainSkeletonEntity,
    )>,
    target_combatants: Query<(&MainSkeletonEntity, &HitboxRadius)>,
    transforms: Query<&Transform>,
    bevy_transmitter: ResMut<BevyTransmitter>,
) {
    // enqueue model actions from action result
    while let Some((action_user_id, action_results)) = new_action_results.0.pop_front() {
        let _result = bevy_transmitter
            .0
            .send(MessageFromBevy::StartedProcessingTurnResult(action_user_id));
        let combatant_entity = combatants_by_id
            .0
            .get(&action_user_id)
            .expect("to have registered the entity");

        let (
            mut transform_manager,
            mut model_action_queue,
            mut action_results_processing,
            skeleton_entity,
        ) = combatants
            .get_mut(*combatant_entity)
            .expect("to have the entity");

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
                            set_non_melee_ability_destination_transform_and_rotation(
                                &action_result,
                                &mut transform_manager,
                                &combatants_by_id.0,
                                skeleton_entity.0,
                                &target_combatants,
                                &transforms,
                            )
                        }
                    }
                    let model_action = match ability_name {
                        // attack is only used by the client to show a generic menu option which is
                        // interpreted as one of the more specific attack types handled below
                        CombatantAbilityNames::Attack => CombatantModelActions::AttackMeleeMainHand,
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
                CombatAction::ConsumableUsed(_) => {
                    info!("pushing consumable used model action");
                    // ask yew what item this is
                    model_action_queue
                        .0
                        .push_back(CombatantModelActions::UseConsumable)
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
    }
}
