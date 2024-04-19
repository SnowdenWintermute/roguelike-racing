use crate::bevy_app::modular_character_plugin::Animations;
use crate::bevy_app::utils::link_animations::AnimationEntityLink;
use crate::frontend_common::CombatantSpecies;
use bevy::prelude::*;

use super::attack_melee_main_hand::UNKNOWN_ANIMATION_DURATION;
use super::{Timestamp, TransformManager};
const TIME_TO_SHOW_HP_CHANGE_NUMBER: u64 = 2000;

pub fn hit_recovery_processor(
    commands: &mut Commands,
    entity: Entity,
    skeleton_entity_transform: &Transform,
    skeleton_entity: Entity,
    combatant_species: &CombatantSpecies,
    animation_managers: &mut Query<&mut TransformManager>,
    transforms: &mut Query<&mut Transform>,
    elapsed: u64,
    current_time: Timestamp,
    animations: &Res<Animations>,
    animation_players: &mut Query<&mut AnimationPlayer>,
    animation_player_links: &Query<&AnimationEntityLink>,
    assets_animation_clips: &Res<Assets<AnimationClip>>,
    transition_started: bool,
) {
    // // translate their floating numbers
    // // check if their hit recovery animation is done and floating numbers done and if so remove this from list of active
    // // model actions
    // let percent_completed = if let Some(animation_name) =
    //     get_animation_name_from_model_action(combatant_species, &CombatantModelActions::HitRecovery)
    // {
    //     let animation_player_link = animation_player_links
    //         .get(skeleton_entity)
    //         .expect("the skeleton to have an animation player link");
    //     let animation_player = animation_players
    //         .get(animation_player_link.0)
    //         .expect("the skeleton's animation entity link to have an animation player");
    //     let animation_handle = animations
    //         .0
    //         .get(&animation_name)
    //         .expect("to have this animation registered");
    //     let animation_clip = assets_animation_clips
    //         .get(animation_handle)
    //         .expect("to have the clip");
    //     animation_player.elapsed() / animation_clip.duration()
    // } else {
    //     elapsed as f32 / UNKNOWN_ANIMATION_DURATION as f32
    // };

    // let hit_recovery_animation_completed = percent_completed >= 1.0;

    // let mut animation_manager = animation_managers
    //     .get_mut(entity)
    //     .expect("entity to have an animation manager");

    // if all_numbers_completed && hit_recovery_animation_completed {
    //     animation_manager
    //         .active_model_actions
    //         .remove(&CombatantModelActions::HitRecovery);
    // }
}
