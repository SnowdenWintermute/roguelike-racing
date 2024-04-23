use crate::bevy_app::modular_character_plugin::process_combatant_model_actions::start_idle_animation::start_idle_animation;
use crate::bevy_app::modular_character_plugin::spawn_combatant::CombatantPropertiesComponent;
use crate::bevy_app::modular_character_plugin::spawn_combatant::CombatantSpeciesComponent;
use crate::bevy_app::modular_character_plugin::spawn_combatant::MainSkeletonBonesAndArmature;
use crate::bevy_app::modular_character_plugin::spawn_combatant::MainSkeletonEntity;
use crate::bevy_app::modular_character_plugin::Animations;
use crate::bevy_app::utils::link_animations::AnimationEntityLink;
use bevy::prelude::*;
use super::ActiveModelActions;
use super::ModelActionQueue;

pub fn start_new_model_actions_or_idle(
    mut combatants: Query<
        (
            &MainSkeletonBonesAndArmature, // to ensure skeleton is assigned already
            &MainSkeletonEntity,
            &CombatantPropertiesComponent,
            &mut ActiveModelActions,
            &mut ModelActionQueue,
        ),
        Or<(
            Changed<ModelActionQueue>,
            Changed<ActiveModelActions>,
            Added<MainSkeletonBonesAndArmature>,
        )>,
    >,
    species_query: Query<&CombatantSpeciesComponent>,
    mut animation_players: Query<&mut AnimationPlayer>,
    animation_player_links: Query<&AnimationEntityLink>,
    animations: Res<Animations>,
) {
    for (
        _,
        skeleton_entity,
        combatant_properties_component,
        mut active_model_actions,
        mut model_action_queue,
    ) in &mut combatants
    {
        let species = species_query
            .get(skeleton_entity.0)
            .expect("skeleton to have a species");
        if model_action_queue.0.len() > 0 && active_model_actions.0.len() == 0 {
            // if no active actions take the next one
            model_action_queue.start_next_model_action(
                &mut active_model_actions,
                &animation_player_links,
                &mut animation_players,
                &animations,
                skeleton_entity.0,
                &species.0,
                &combatant_properties_component.0,
                500,
            );
        } else if active_model_actions.0.len() == 0 {
            if combatant_properties_component.0.hit_points == 0 {
                continue;
            }
            start_idle_animation(
                &animation_player_links,
                &mut animation_players,
                &animations,
                &species.0,
                skeleton_entity.0,
                &combatant_properties_component.0,
            );
        }
    }
}
