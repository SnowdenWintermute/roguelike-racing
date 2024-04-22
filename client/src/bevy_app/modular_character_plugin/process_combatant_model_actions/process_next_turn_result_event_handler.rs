use super::enqueue_approach_melee_target_model_action::enqueue_approach_melee_target_model_action;
use super::model_actions::CombatantModelActions;
use super::ModelActionQueue;
use super::TransformManager;
use crate::bevy_app::modular_character_plugin::spawn_combatant::ActionResultsProcessing;
use crate::bevy_app::modular_character_plugin::spawn_combatant::HitboxRadius;
use crate::bevy_app::modular_character_plugin::spawn_combatant::MainSkeletonEntity;
use crate::bevy_app::modular_character_plugin::CombatantsById;
use crate::bevy_app::modular_character_plugin::HomeLocation;
use crate::bevy_app::modular_character_plugin::TurnResultsQueue;
use bevy::prelude::*;
use common::combat::combat_actions::CombatAction;
use common::combat::CombatTurnResult;
use common::combatants::abilities::CombatantAbilityNames;

pub fn process_next_turn_result_event_handler(
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
) {
    if let Some(CombatTurnResult {
        combatant_id,
        action_results,
    }) = turn_results_queue.0.pop_front()
    {
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
                    // if melee, queue up the approach model action
                    if ability_name.get_attributes().is_melee && i == 0 {
                        enqueue_approach_melee_target_model_action(
                            &action_result,
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
