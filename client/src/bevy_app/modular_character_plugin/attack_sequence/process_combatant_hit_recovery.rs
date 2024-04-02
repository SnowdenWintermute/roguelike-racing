use crate::bevy_app::modular_character_plugin::animation_manager_component::ActionSequenceStates;
use crate::bevy_app::modular_character_plugin::animation_manager_component::AnimationManagerComponent;
use crate::bevy_app::modular_character_plugin::Animations;
use crate::frontend_common::animation_names::AnimationType;
use crate::frontend_common::animation_names::CombatantAnimations;
use crate::frontend_common::CombatantSpecies;
use bevy::prelude::*;

pub const TIME_TO_SHOW_HP_CHANGE_NUMBER: u64 = 1000;

pub fn process_combatant_hit_recovery(
    commands: &mut Commands,
    animation_manager: &mut AnimationManagerComponent,
    current_time: u64,
    animation_player: &mut AnimationPlayer,
    animations: &Res<Animations>,
    assets_animation_clips: &Res<Assets<AnimationClip>>,
    transforms: &mut Query<&mut Transform>,
    species: &CombatantSpecies,
) {
    let anim_name = species.animation_name(AnimationType::HitRecovery);

    let animation_handle = animations
        .0
        .get(&anim_name)
        .expect("to have this animation registered");
    let animation_clip = assets_animation_clips
        .get(animation_handle)
        .expect("to have the clip");
    let percent_completed = animation_player.elapsed() / animation_clip.duration();
    let hit_recovery_animation_completed = percent_completed >= 1.0;

    let mut all_numbers_completed = true;
    for (i, hp_change_number) in animation_manager
        .hp_change_numbers
        .clone()
        .iter()
        .enumerate()
    {
        if let Ok(mut transform) = transforms.get_mut(hp_change_number.entity) {
            let elapsed = current_time - hp_change_number.time_started;
            let number_showing_percent_completed =
                elapsed as f32 / TIME_TO_SHOW_HP_CHANGE_NUMBER as f32;

            transform.translation = hp_change_number.home_location.translation.lerp(
                hp_change_number.destination.translation,
                number_showing_percent_completed,
            );

            if number_showing_percent_completed >= 1.0 {
                animation_manager.hp_change_numbers.remove(i);
                let billboard_entity_commands = commands.entity(hp_change_number.entity);
                billboard_entity_commands.despawn_recursive();
            } else {
                all_numbers_completed = false;
            }
        };
    }

    if all_numbers_completed && hit_recovery_animation_completed {
        animation_manager
            .active_states
            .remove(&ActionSequenceStates::HitRecovery);
    }
}
