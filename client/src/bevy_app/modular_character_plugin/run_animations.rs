use super::spawn_combatant::CombatantSpeciesComponent;
use super::Animations;
use crate::bevy_app::utils::link_animations::AnimationEntityLink;
use crate::frontend_common::animation_names::AnimationType;
use crate::frontend_common::animation_names::CombatantAnimations;
use bevy::prelude::*;

pub fn run_animations(
    mut animation_player_query: Query<&mut AnimationPlayer>,
    animation_player_link_query: Query<
        (&AnimationEntityLink, &CombatantSpeciesComponent),
        Added<AnimationEntityLink>,
    >,
    animations: Res<Animations>,
) {
    for (animation_player_entity_link, species) in animation_player_link_query.iter() {
        let mut animation_player = animation_player_query
            .get_mut(animation_player_entity_link.0)
            .expect("to have an animation player on the main skeleton");

        let idle_animation_name = species.0.animation_name(AnimationType::Idle);
        info!("RUNNING ANIMATIONS, {idle_animation_name}");

        animation_player
            .play(
                animations
                    .0
                    .get(&idle_animation_name)
                    .expect("to have an animation by this name")
                    .clone_weak(),
            )
            .repeat()
            .set_speed(0.5);
    }
}
