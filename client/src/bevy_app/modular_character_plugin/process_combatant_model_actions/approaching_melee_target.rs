use crate::bevy_app::modular_character_plugin::animation_manager_component::AnimationManagerComponent;
use crate::bevy_app::modular_character_plugin::handle_combat_turn_results::combatant_model_actions::CombatantModelActions;
use crate::bevy_app::modular_character_plugin::Animations;
use crate::bevy_app::utils::rotate_transform_toward_target::rotate_transform_toward_target;
use crate::bevy_app::utils::translate_transform_toward_target::translate_transform_toward_target;
use crate::frontend_common::CombatantSpecies;
use bevy::math::u64;
use bevy::prelude::*;

const TIME_TO_TRANSLATE: u64 = 1500;
const TIME_TO_ROTATE: u64 = 1000;
const PERCENT_DISTANCE_TO_START_TRANSITION: f32 = 0.8;

pub fn combatant_approaching_melee_target_processor(
    skeleton_entity_transform: &mut Transform,
    animation_manager: &mut AnimationManagerComponent,
    home_location: &Transform,
    elapsed: u64,
) {
    // move toward destination
    let percent_distance_travelled = translate_transform_toward_target(
        skeleton_entity_transform,
        home_location,
        &animation_manager.destination.expect("a destination"),
        elapsed,
        TIME_TO_TRANSLATE,
    );
    if let Some(target_rotation) = animation_manager.target_rotation {
        rotate_transform_toward_target(
            skeleton_entity_transform,
            &home_location.rotation,
            &target_rotation,
            elapsed,
            TIME_TO_ROTATE,
        );
    }

    let transition_started = animation_manager
        .active_model_actions
        .get(&CombatantModelActions::ApproachMeleeTarget)
        .expect("this model action to be active")
        .transition_started;

    if percent_distance_travelled >= PERCENT_DISTANCE_TO_START_TRANSITION && !transition_started {
        // start next model action and mark this one's transition as started
        animation_manager.start_next_model_action(
            animation_player_links,
            animation_players,
            animations,
            skeleton_entity,
            combatant_species,
        );
        animation_manager
            .active_model_actions
            .get_mut(&CombatantModelActions::ApproachMeleeTarget)
            .expect("this model action to be active")
            .transition_started = true;

        // animation_manager
        //     .active_model_actions
        //     .insert(ActionSequenceStates::Swinging, Some(current_time));

        // let attack_anim_name = species.animation_name(AnimationType::Attack);

        // // - start playing swing animation
        // let animation_handle = animations
        //     .0
        //     .get(&attack_anim_name)
        //     .expect("to have this animation");
        // animation_player.play_with_transition(animation_handle.clone(), Duration::from_millis(500));
    }
    // - if reached destination, deactivate approaching
    if percent_distance_travelled >= 1.0 {
        animation_manager
            .active_model_actions
            .remove(&CombatantModelActions::ApproachMeleeTarget);
    }
}
