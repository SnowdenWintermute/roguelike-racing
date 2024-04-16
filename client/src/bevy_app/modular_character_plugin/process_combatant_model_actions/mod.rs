mod approaching_melee_target;
mod start_idle_animation;
use self::approaching_melee_target::combatant_approaching_melee_target_processor;
use super::animation_manager_component::AnimationManagerComponent;
use super::handle_combat_turn_results::combatant_model_actions::get_animation_name_from_model_action;
// use super::handle_combat_turn_results::combatant_model_actions::CombatantModelActionProgressTracker;
use super::handle_combat_turn_results::combatant_model_actions::CombatantModelActions;
use super::spawn_combatant::CombatantActionResultsManagerComponent;
use super::spawn_combatant::CombatantIdComponent;
use super::spawn_combatant::CombatantSpeciesComponent;
use super::spawn_combatant::HitboxRadius;
use super::spawn_combatant::MainSkeletonBonesAndArmature;
use super::spawn_combatant::MainSkeletonEntity;
use super::Animations;
// use super::CombatantsById;
use super::HomeLocation;
use crate::bevy_app::modular_character_plugin::process_combatant_model_actions::start_idle_animation::start_idle_animation;
use crate::bevy_app::utils::link_animations::AnimationEntityLink;
use crate::frontend_common::CombatantSpecies;
use bevy::prelude::*;
use js_sys::Date;

pub fn process_combatant_model_actions(
    // combatants_by_id: Res<CombatantsById>,
    mut combatants: Query<
        (
            &MainSkeletonBonesAndArmature, // to ensure skeleton is assigned already
            &CombatantIdComponent,
            &MainSkeletonEntity,
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
    // species_query: Query<&CombatantSpeciesComponent>,
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
        hitbox_radius,
        home_location,
        mut animation_manager_component,
        action_result_manager,
    ) in &mut combatants
    {
        let species = species_query
            .get(skeleton_entity.0)
            .expect("skeleton to have a species");
        if animation_manager_component.model_action_queue.len() > 0 {
            // if no active actions take the next one
            animation_manager_component.start_next_model_action(
                &animation_player_links,
                &mut animation_players,
                &animations,
                skeleton_entity.0,
                &species.0,
                500,
            );
        } // process all active actions
        let now = Date::new_0().get_time() as u64;
        for (model_action, model_action_progress_tracker) in
            animation_manager_component.active_model_actions.clone()
        {
            let elapsed = now - model_action_progress_tracker.time_started;
            let mut skeleton_entity_transform = transforms
                .get_mut(skeleton_entity.0)
                .expect("the skeleton entity to have a transform");
            process_model_action(
                &model_action,
                &mut skeleton_entity_transform,
                skeleton_entity.0,
                &species.0,
                &mut animation_manager_component,
                &home_location.0,
                elapsed,
                &animations,
                &mut animation_players,
                &animation_player_links,
            );
        }
        // if no model actions remaining, start idle animation
        if animation_manager_component.active_model_actions.len() == 0 {
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

pub fn process_model_action(
    model_action: &CombatantModelActions,
    skeleton_entity_transform: &mut Transform,
    skeleton_entity: Entity,
    combatant_species: &CombatantSpecies,
    animation_manager: &mut AnimationManagerComponent,
    home_location: &Transform,
    elapsed: u64,
    animations: &Res<Animations>,
    animation_players: &mut Query<&mut AnimationPlayer>,
    animation_player_links: &Query<&AnimationEntityLink>,
) {
    match model_action {
        CombatantModelActions::ApproachMeleeTarget => combatant_approaching_melee_target_processor(
            skeleton_entity_transform,
            skeleton_entity,
            combatant_species,
            animation_manager,
            home_location,
            elapsed,
            animations,
            animation_players,
            animation_player_links,
        ),
        CombatantModelActions::ReturnHome => todo!(),
        CombatantModelActions::Recenter => todo!(),
        CombatantModelActions::TurnToFaceTarget => todo!(),
        CombatantModelActions::AttackMeleeMainHand => todo!(),
        CombatantModelActions::AttackMeleeOffHand => todo!(),
        CombatantModelActions::HitRecovery => todo!(),
        CombatantModelActions::Death => todo!(),
        CombatantModelActions::Idle => todo!(),
    }
}
