use super::combatant_model_actions::CombatantModelActions;
use super::enqueue_approach_melee_target_model_action::enqueue_approach_melee_target_model_action;
use crate::bevy_app::modular_character_plugin::animation_manager_component::AnimationManagerComponent;
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
            &CombatantActionResultsManagerComponent,
            &mut AnimationManagerComponent,
            &MainSkeletonEntity,
            &HomeLocation,
        ),
        Changed<CombatantActionResultsManagerComponent>,
    >,
    target_combatants: Query<(&MainSkeletonEntity, &HitboxRadius)>,
    combatants_by_id: Res<CombatantsById>,
    transforms: Query<&Transform>,
) {
    for (action_result_manager, mut animation_manager, skeleton_entity, home_location) in
        &mut combatants
    {
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
                            &mut animation_manager,
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

                    animation_manager.model_action_queue.push_back(model_action)
                }
                CombatAction::ConsumableUsed(_) => todo!(),
            }

            // if this is the last action in the action_results_queue, send their model home
            if action_result_manager.action_result_queue.len() < 1 {
                animation_manager
                    .model_action_queue
                    .push_back(CombatantModelActions::ReturnHome);
                animation_manager
                    .model_action_queue
                    .push_back(CombatantModelActions::Recenter);
            }

            info!(
                "enqueued model actions {:?}",
                animation_manager.model_action_queue
            );
        }
    }
}
