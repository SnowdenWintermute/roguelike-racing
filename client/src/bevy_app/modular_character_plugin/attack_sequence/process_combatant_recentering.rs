use super::rotate_transform_toward_target::rotate_transform_toward_target;
use crate::bevy_app::modular_character_plugin::animation_manager_component::ActionSequenceStates;
use crate::bevy_app::modular_character_plugin::animation_manager_component::AnimationManagerComponent;
use bevy::math::u64;
use bevy::prelude::*;

pub const TIME_TO_RECENTER: u64 = 1000;

pub fn process_combatant_recentering(
    skeleton_entity_transform: &mut Transform,
    animation_manager: &mut AnimationManagerComponent,
    home_location: &Transform,
    elapsed: u64,
) {
    let percent_rotated = rotate_transform_toward_target(
        skeleton_entity_transform,
        &animation_manager
            .target_rotation
            .expect("to have saved the location"),
        &home_location.rotation,
        elapsed,
        TIME_TO_RECENTER,
    );

    if percent_rotated >= 1.0 {
        animation_manager
            .active_states
            .remove(&ActionSequenceStates::Recentering);

        animation_manager.last_location = None;
        animation_manager.destination = None;
    }
}
