use super::rotate_transform_toward_target::rotate_transform_toward_target;
use super::translate_transform_toward_target::translate_transform_toward_target;
use crate::bevy_app::modular_character_plugin::animation_manager_component::ActionSequenceStates;
use crate::bevy_app::modular_character_plugin::animation_manager_component::AnimationManagerComponent;
use crate::bevy_app::modular_character_plugin::Animations;
use crate::frontend_common::animation_names::AnimationType;
use crate::frontend_common::animation_names::CombatantAnimations;
use crate::frontend_common::CombatantSpecies;
use bevy::math::u64;
use bevy::prelude::*;
use std::time::Duration;

const TIME_TO_TRANSLATE: u64 = 1500;
const TIME_TO_ROTATE: u64 = 1000;
const PERCENT_DISTANCE_TO_START_WEAPON_SWING: f32 = 0.8;

pub fn process_combatant_approaching_melee_target(
    skeleton_entity_transform: &mut Transform,
    animation_manager: &mut AnimationManagerComponent,
    home_location: &Transform,
    elapsed: u64,
    animation_player: &mut AnimationPlayer,
    animations: &Res<Animations>,
    current_time: u64,
    species: &CombatantSpecies,
) {
    // approaching
    // - move toward destination
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
    // - if within threshold and if not already swinging, activate swinging state
    if percent_distance_travelled >= PERCENT_DISTANCE_TO_START_WEAPON_SWING
        && !animation_manager
            .active_states
            .contains_key(&ActionSequenceStates::Swinging)
    {
        animation_manager
            .active_states
            .insert(ActionSequenceStates::Swinging, Some(current_time));

        let attack_anim_name = species.animation_name(AnimationType::Attack);

        // - start playing swing animation
        let animation_handle = animations
            .0
            .get(&attack_anim_name)
            .expect("to have this animation");
        animation_player.play_with_transition(animation_handle.clone(), Duration::from_millis(500));
    }
    // - if reached destination, deactivate approaching
    if percent_distance_travelled >= 1.0 {
        animation_manager
            .active_states
            .remove(&ActionSequenceStates::ApproachingTarget);
    }
}
