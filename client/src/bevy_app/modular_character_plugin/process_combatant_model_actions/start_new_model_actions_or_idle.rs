use crate::bevy_app::modular_character_plugin::animation_manager_component::AnimationManagerComponent;
use crate::bevy_app::modular_character_plugin::process_combatant_model_actions::start_idle_animation::start_idle_animation;
use crate::bevy_app::modular_character_plugin::spawn_combatant::CombatantActionResultsManagerComponent;
use crate::bevy_app::modular_character_plugin::spawn_combatant::CombatantEquipment;
use crate::bevy_app::modular_character_plugin::spawn_combatant::CombatantIdComponent;
use crate::bevy_app::modular_character_plugin::spawn_combatant::CombatantSpeciesComponent;
use crate::bevy_app::modular_character_plugin::spawn_combatant::HitboxRadius;
use crate::bevy_app::modular_character_plugin::spawn_combatant::MainSkeletonBonesAndArmature;
use crate::bevy_app::modular_character_plugin::spawn_combatant::MainSkeletonEntity;
use crate::bevy_app::modular_character_plugin::Animations;
use crate::bevy_app::modular_character_plugin::HomeLocation;
use crate::bevy_app::utils::link_animations::AnimationEntityLink;
use crate::frontend_common::CombatantSpecies;
use bevy::prelude::*;
use js_sys::Date;

pub fn start_new_model_actions_or_idle(
    mut combatants: Query<
        (
            &MainSkeletonBonesAndArmature, // to ensure skeleton is assigned already
            &CombatantIdComponent,
            &MainSkeletonEntity,
            &CombatantEquipment,
            &HitboxRadius,
            &HomeLocation,
            &mut AnimationManagerComponent,
            &mut CombatantActionResultsManagerComponent,
        ),
        Or<(
            Changed<AnimationManagerComponent>,
            Added<MainSkeletonBonesAndArmature>,
        )>,
    >,
    species_query: Query<&CombatantSpeciesComponent>,
    mut animation_players: Query<&mut AnimationPlayer>,
    animation_player_links: Query<&AnimationEntityLink>,
    animations: Res<Animations>,
    mut transforms: Query<&mut Transform>,
) {
    for (
        _,
        combatant_id_component,
        skeleton_entity,
        equipment_component,
        hitbox_radius,
        home_location,
        mut animation_manager_component,
        action_result_manager,
    ) in &mut combatants
    {
        let species = species_query
            .get(skeleton_entity.0)
            .expect("skeleton to have a species");
        if animation_manager_component.model_action_queue.len() > 0
            && animation_manager_component.active_model_actions.len() == 0
        {
            // if no active actions take the next one
            animation_manager_component.start_next_model_action(
                &animation_player_links,
                &mut animation_players,
                &animations,
                skeleton_entity.0,
                &species.0,
                &equipment_component.0,
                500,
            );
        } else if animation_manager_component.active_model_actions.len() == 0 {
            start_idle_animation(
                &animation_player_links,
                &mut animation_players,
                &animations,
                &species.0,
                skeleton_entity.0,
            );
        }
    }
}
