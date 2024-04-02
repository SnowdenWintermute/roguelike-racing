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

const TIME_TO_RETURN: u64 = 1000;
const PERCENT_DISTANCE_TO_START_IDLE: f32 = 0.8;

pub fn process_combatant_returning_to_home_position(
    skeleton_entity_transform: &mut Transform,
    animation_manager: &mut AnimationManagerComponent,
    home_location: &Transform,
    elapsed: u64,
    animation_player: &mut AnimationPlayer,
    animations: &Res<Animations>,
    current_time: u64,
    species: &CombatantSpecies,
) {
    // - move toward destination
    let percent_distance_travelled = translate_transform_toward_target(
        skeleton_entity_transform,
        &animation_manager
            .last_location
            .expect("to have saved the prev location"),
        &home_location,
        elapsed,
        TIME_TO_RETURN,
    );
    // - if within threshold and if not already swinging, activate swinging state
    if percent_distance_travelled >= PERCENT_DISTANCE_TO_START_IDLE
        && !animation_manager
            .active_states
            .contains_key(&ActionSequenceStates::Recentering)
    {
        let anim_name = species.animation_name(AnimationType::Idle);
        // - start playing animation
        let animation_handle = animations
            .0
            .get(&anim_name)
            .expect("to have this animation");
        animation_player
            .play_with_transition(animation_handle.clone(), Duration::from_millis(1000))
            .repeat();
    }
    // - if reached destination, deactivate approaching
    if percent_distance_travelled >= 1.0 {
        animation_manager
            .active_states
            .remove(&ActionSequenceStates::Returning);
        animation_manager
            .active_states
            .insert(ActionSequenceStates::Recentering, Some(current_time));
    }
}
