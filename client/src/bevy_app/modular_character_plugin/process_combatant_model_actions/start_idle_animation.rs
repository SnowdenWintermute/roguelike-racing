use std::collections::HashMap;

use crate::bevy_app::modular_character_plugin::Animations;
use crate::bevy_app::utils::link_animations::AnimationEntityLink;
use crate::frontend_common::CombatantSpecies;
use bevy::animation::AnimationPlayer;
use bevy::prelude::*;
use common::items::{equipment::EquipmentSlots, Item};

use super::model_actions::{get_animation_name_from_model_action, CombatantModelActions};

pub fn start_idle_animation(
    animation_player_links: &Query<&AnimationEntityLink>,
    animation_players: &mut Query<&mut AnimationPlayer>,
    animations: &Res<Animations>,
    species: &CombatantSpecies,
    skeleton_entity: Entity,
    equipment: &HashMap<EquipmentSlots, Item>,
) {
    if let Ok(animation_player_entity_link) = animation_player_links.get(skeleton_entity) {
        let mut animation_player = animation_players
            .get_mut(animation_player_entity_link.0)
            .expect("to have an animation player on the main skeleton");

        if let Some(idle_animation_name) =
            get_animation_name_from_model_action(&species, &CombatantModelActions::Idle, equipment)
        {
            animation_player
                .play(
                    animations
                        .0
                        .get(&idle_animation_name)
                        .expect("to have an animation by this name")
                        .clone_weak(),
                )
                .repeat();
        };
    };
}
