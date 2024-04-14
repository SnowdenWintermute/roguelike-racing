use crate::bevy_app::modular_character_plugin::animation_manager_component::AnimationManagerComponent;
use crate::bevy_app::modular_character_plugin::spawn_combatant::CombatantActionResultManagerComponent;
use bevy::prelude::*;
use common::combat::combat_actions::CombatAction;
use common::combatants::abilities::CombatantAbilityNames;

pub fn process_action_results(
    combatant_action_result_managers: Query<
        &CombatantActionResultManagerComponent,
        Changed<CombatantActionResultManagerComponent>,
    >,
    mut combatant_animation_managers: Query<&mut AnimationManagerComponent>,
) {
    for action_result_manager in &combatant_action_result_managers {
        if let Some(current_action_result_processing) =
            &action_result_manager.0.current_action_result_processing
        {
            match &current_action_result_processing.action {
                CombatAction::AbilityUsed(ability_name) => match ability_name {
                    CombatantAbilityNames::Attack => (),
                    CombatantAbilityNames::AttackMeleeMainhand => {
                        // send CombatantActionEvent(EventType::AttackMeleeMainHand)
                        // initiate attack animation sequence
                    }
                    CombatantAbilityNames::AttackMeleeOffhand => todo!(),
                    CombatantAbilityNames::AttackRangedMainhand => todo!(),
                    CombatantAbilityNames::Fire => todo!(),
                    CombatantAbilityNames::Ice => todo!(),
                    CombatantAbilityNames::Healing => todo!(),
                },
                CombatAction::ConsumableUsed(_) => todo!(),
            }
        }
    }
}
