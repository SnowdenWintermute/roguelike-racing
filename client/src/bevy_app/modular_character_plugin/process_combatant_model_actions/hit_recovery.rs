use bevy::prelude::*;
use crate::bevy_app::modular_character_plugin::animation_manager_component::{AnimationManagerComponent, Timestamp};
use crate::bevy_app::modular_character_plugin::handle_combat_turn_results::combatant_model_actions::{get_animation_name_from_model_action, CombatantModelActions};
use crate::bevy_app::modular_character_plugin::Animations;
use crate::bevy_app::utils::link_animations::AnimationEntityLink;
use crate::frontend_common::CombatantSpecies;

use super::attack_melee_main_hand::UNKNOWN_ANIMATION_DURATION;
const TIME_TO_SHOW_HP_CHANGE_NUMBER: u64 = 2000;

pub fn hit_recovery_processor(
    commands: &mut Commands,
    entity: Entity,
    skeleton_entity_transform: &Transform,
    skeleton_entity: Entity,
    combatant_species: &CombatantSpecies,
    animation_managers: &mut Query<&mut AnimationManagerComponent>,
    transforms: &mut Query<&mut Transform>,
    elapsed: u64,
    current_time: Timestamp,
    animations: &Res<Animations>,
    animation_players: &mut Query<&mut AnimationPlayer>,
    animation_player_links: &Query<&AnimationEntityLink>,
    assets_animation_clips: &Res<Assets<AnimationClip>>,
    transition_started: bool,
) {
    // translate their floating numbers
    // check if their hit recovery animation is done and floating numbers done and if so remove this from list of active
    // model actions
    let percent_completed = if let Some(animation_name) =
        get_animation_name_from_model_action(combatant_species, &CombatantModelActions::HitRecovery)
    {
        let animation_player_link = animation_player_links
            .get(skeleton_entity)
            .expect("the skeleton to have an animation player link");
        let animation_player = animation_players
            .get(animation_player_link.0)
            .expect("the skeleton's animation entity link to have an animation player");
        let animation_handle = animations
            .0
            .get(&animation_name)
            .expect("to have this animation registered");
        let animation_clip = assets_animation_clips
            .get(animation_handle)
            .expect("to have the clip");
        animation_player.elapsed() / animation_clip.duration()
    } else {
        elapsed as f32 / UNKNOWN_ANIMATION_DURATION as f32
    };

    let hit_recovery_animation_completed = percent_completed >= 1.0;

    let mut animation_manager = animation_managers
        .get_mut(entity)
        .expect("entity to have an animation manager");
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
            .active_model_actions
            .remove(&CombatantModelActions::HitRecovery);
    }
}
