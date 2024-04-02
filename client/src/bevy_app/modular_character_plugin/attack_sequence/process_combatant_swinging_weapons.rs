use crate::bevy_app::modular_character_plugin::animation_manager_component::ActionSequenceStates;
use crate::bevy_app::modular_character_plugin::animation_manager_component::AnimationManagerComponent;
use crate::bevy_app::modular_character_plugin::Animations;
use crate::bevy_app::modular_character_plugin::HitRecoveryActivationEvent;
use crate::frontend_common::animation_names::AnimationType;
use crate::frontend_common::animation_names::CombatantAnimations;
use crate::frontend_common::CombatantSpecies;
use bevy::math::u64;
use bevy::prelude::*;
use std::time::Duration;

pub const SWORD_SLASH_PERCENT_COMPLETE_TRANSITION_THRESHOLD: f32 = 0.65;
pub const SWORD_SLASH_HIT_ACTIVATION_PERCENT_COMPLETION: f32 = 0.45;

pub fn process_combatant_swinging_weapons(
    animation_manager: &mut AnimationManagerComponent,
    home_location: &Transform,
    animation_player: &mut AnimationPlayer,
    animations: &Res<Animations>,
    assets_animation_clips: &Res<Assets<AnimationClip>>,
    current_time: u64,
    hit_recovery_activation_event_writer: &mut EventWriter<HitRecoveryActivationEvent>,
    species: &CombatantSpecies,
) {
    let anim_name = species.animation_name(AnimationType::Attack);
    // - if duration threshold passed, activate returning
    let animation_handle = animations
        .0
        .get(&anim_name)
        .expect("to have this animation registered");
    let animation_clip = assets_animation_clips
        .get(animation_handle)
        .expect("to have the clip");
    let percent_completed = animation_player.elapsed() / animation_clip.duration();

    if percent_completed >= SWORD_SLASH_HIT_ACTIVATION_PERCENT_COMPLETION {
        if let Some(current_targets) = animation_manager.current_targets.take() {
            hit_recovery_activation_event_writer.send(HitRecoveryActivationEvent(Vec::from([(
                current_targets[0],
                10,
            )])));
        }
    }

    if percent_completed >= SWORD_SLASH_PERCENT_COMPLETE_TRANSITION_THRESHOLD {
        animation_manager
            .active_states
            .remove(&ActionSequenceStates::Swinging);

        animation_manager
            .active_states
            .insert(ActionSequenceStates::Returning, Some(current_time));
        // set new destination as home location and save prev location
        animation_manager.last_location = animation_manager.destination.take();
        animation_manager.destination = Some(home_location.clone());

        let anim_name = species.animation_name(AnimationType::ReturningToHome);
        let animation_handle = animations
            .0
            .get(&anim_name)
            .expect("to have this animation");
        animation_player
            .play_with_transition(animation_handle.clone(), Duration::from_millis(500))
            .repeat();
    }
}
