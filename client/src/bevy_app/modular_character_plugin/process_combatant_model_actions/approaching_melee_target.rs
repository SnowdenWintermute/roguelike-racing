use std::collections::HashMap;

use crate::bevy_app::modular_character_plugin::animation_manager_component::AnimationManagerComponent;
use crate::bevy_app::modular_character_plugin::handle_combat_turn_results::combatant_model_actions::CombatantModelActions;
use crate::bevy_app::modular_character_plugin::Animations;
use crate::bevy_app::utils::link_animations::AnimationEntityLink;
use crate::bevy_app::utils::rotate_transform_toward_target::rotate_transform_toward_target;
use crate::bevy_app::utils::translate_transform_toward_target::translate_transform_toward_target;
use crate::frontend_common::CombatantSpecies;
use bevy::math::u64;
use bevy::prelude::*;
use common::items::equipment::EquipmentSlots;
use common::items::Item;

const TIME_TO_TRANSLATE: u64 = 1500;
const TIME_TO_ROTATE: u64 = 1000;
const PERCENT_DISTANCE_TO_START_TRANSITION: f32 = 0.8;

pub fn combatant_approaching_melee_target_processor(
    skeleton_entity_transform: &mut Transform,
    skeleton_entity: Entity,
    combatant_species: &CombatantSpecies,
    equipment: &HashMap<EquipmentSlots, Item>,
    animation_manager: &mut AnimationManagerComponent,
    home_location: &Transform,
    elapsed: u64,
    animations: &Res<Animations>,
    animation_players: &mut Query<&mut AnimationPlayer>,
    animation_player_links: &Query<&AnimationEntityLink>,
    transition_started: bool,
) {
    // move toward destination
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

    if percent_distance_travelled >= PERCENT_DISTANCE_TO_START_TRANSITION && !transition_started {
        // start next model action and mark this one's transition as started
        animation_manager.start_next_model_action(
            animation_player_links,
            animation_players,
            animations,
            skeleton_entity,
            combatant_species,
            equipment,
            500,
        );
        animation_manager
            .active_model_actions
            .get_mut(&CombatantModelActions::ApproachMeleeTarget)
            .expect("this model action to be active")
            .transition_started = true;
    }
    // - if reached destination, deactivate approaching
    if percent_distance_travelled >= 1.0 {
        animation_manager
            .active_model_actions
            .remove(&CombatantModelActions::ApproachMeleeTarget);
    }
}
