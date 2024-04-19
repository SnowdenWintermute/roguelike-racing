use super::enqueue_approach_melee_target_model_action::enqueue_approach_melee_target_model_action;
use crate::bevy_app::modular_character_plugin::process_combatant_model_actions::model_actions::CombatantModelActions;
use crate::bevy_app::modular_character_plugin::process_combatant_model_actions::ModelActionQueue;
use crate::bevy_app::modular_character_plugin::process_combatant_model_actions::TransformManager;
use crate::bevy_app::modular_character_plugin::spawn_combatant::CombatantActionResultsManagerComponent;
use crate::bevy_app::modular_character_plugin::spawn_combatant::HitboxRadius;
use crate::bevy_app::modular_character_plugin::spawn_combatant::MainSkeletonEntity;
use crate::bevy_app::modular_character_plugin::CombatantsById;
use crate::bevy_app::modular_character_plugin::HomeLocation;
use bevy::prelude::*;
use common::combat::combat_actions::CombatAction;
use common::combatants::abilities::CombatantAbilityNames;

pub fn enqueue_model_actions_from_action_results(
    mut combatants: Query<
        (
            &mut CombatantActionResultsManagerComponent,
            &mut TransformManager,
            &mut ModelActionQueue,
            &MainSkeletonEntity,
            &HomeLocation,
        ),
        Changed<CombatantActionResultsManagerComponent>,
    >,
    target_combatants: Query<(&MainSkeletonEntity, &HitboxRadius)>,
    combatants_by_id: Res<CombatantsById>,
    transforms: Query<&Transform>,
) {
    for (
        mut action_result_manager,
        mut transform_manager,
        mut model_action_queue,
        skeleton_entity,
        home_location,
    ) in &mut combatants
    {
        if action_result_manager.done_enqueueing_model_actions_for_current_action_result {
            continue;
        }
        if let Some(current_action_result_processing) =
            &action_result_manager.current_action_result_processing
        {
            match &current_action_result_processing.action {
                CombatAction::AbilityUsed(ability_name) => {
                    // if melee, queue up the approach model action
                    let current_transform = transforms
                        .get(skeleton_entity.0)
                        .expect("the skeleton to have a transform");
                    if ability_name.get_attributes().is_melee
                        && home_location.0.translation == current_transform.translation
                    {
                        enqueue_approach_melee_target_model_action(
                            current_action_result_processing,
                            &mut transform_manager,
                            &mut model_action_queue,
                            &combatants_by_id.0,
                            skeleton_entity.0,
                            &target_combatants,
                            &transforms,
                        )
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
                        CombatantAbilityNames::Fire => todo!(),
                        CombatantAbilityNames::Ice => todo!(),
                        CombatantAbilityNames::Healing => todo!(),
                    };

                    model_action_queue.0.push_back(model_action)
                }
                CombatAction::ConsumableUsed(_) => todo!(),
            }

            // if this is the last action in the action_results_queue, send their model home
            if action_result_manager.action_result_queue.len() < 1 {
                model_action_queue
                    .0
                    .push_back(CombatantModelActions::ReturnHome);
                model_action_queue
                    .0
                    .push_back(CombatantModelActions::Recenter);
            }

            action_result_manager.done_enqueueing_model_actions_for_current_action_result = true;

            // info!(
            //     "enqueued model actions {:?}",
            //     model_action_queue.0.model_action_queue
            // );
        }
    }
}
