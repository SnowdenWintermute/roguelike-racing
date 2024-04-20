use crate::bevy_app::{
    modular_character_plugin::Animations, utils::link_animations::AnimationEntityLink,
};
use bevy::prelude::*;

pub fn get_percent_animation_completed(
    skeleton_entity: &Entity,
    animation_player_links: &Query<&AnimationEntityLink>,
    animation_players: &Query<&mut AnimationPlayer>,
    animations: &Res<Animations>,
    assets_animation_clips: &Res<Assets<AnimationClip>>,
    animation_name: &String,
) -> f32 {
    let animation_player_link = animation_player_links
        .get(*skeleton_entity)
        .expect("the skeleton to have an animation player link");
    let animation_player = animation_players
        .get(animation_player_link.0)
        .expect("the skeleton's animation entity link to have an animation player");
    let animation_handle = animations
        .0
        .get(animation_name)
        .expect("to have this animation registered");
    let animation_clip = assets_animation_clips
        .get(animation_handle)
        .expect("to have the clip");
    animation_player.elapsed() / animation_clip.duration()
}
