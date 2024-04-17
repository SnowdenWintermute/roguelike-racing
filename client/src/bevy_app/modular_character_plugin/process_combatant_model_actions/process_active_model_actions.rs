use crate::bevy_app::modular_character_plugin::animation_manager_component::AnimationManagerComponent;
use crate::bevy_app::modular_character_plugin::handle_combat_turn_results::combatant_model_actions::CombatantModelActions;
use crate::bevy_app::modular_character_plugin::spawn_combatant::CombatantActionResultsManagerComponent;
use crate::bevy_app::modular_character_plugin::spawn_combatant::CombatantEquipment;
use crate::bevy_app::modular_character_plugin::spawn_combatant::CombatantIdComponent;
use crate::bevy_app::modular_character_plugin::spawn_combatant::CombatantSpeciesComponent;
use crate::bevy_app::modular_character_plugin::spawn_combatant::HitboxRadius;
use crate::bevy_app::modular_character_plugin::spawn_combatant::MainSkeletonBonesAndArmature;
use crate::bevy_app::modular_character_plugin::spawn_combatant::MainSkeletonEntity;
use crate::bevy_app::modular_character_plugin::Animations;
use crate::bevy_app::modular_character_plugin::CombatantsById;
use crate::bevy_app::modular_character_plugin::HomeLocation;
use crate::bevy_app::utils::link_animations::AnimationEntityLink;
use crate::frontend_common::CombatantSpecies;
use bevy::prelude::*;
use js_sys::Date;

use super::approaching_melee_target::combatant_approaching_melee_target_processor;

pub fn process_active_model_actions(
    mut combatants: Query<(
        &MainSkeletonBonesAndArmature, // to ensure skeleton is assigned already
        &CombatantIdComponent,
        &MainSkeletonEntity,
        &HitboxRadius,
        &HomeLocation,
        &mut AnimationManagerComponent,
        &mut CombatantActionResultsManagerComponent,
        &CombatantEquipment,
    )>,
    combatants_by_id: Res<CombatantsById>,
    species_query: Query<&CombatantSpeciesComponent>,
    animation_managers: Query<&mut AnimationManagerComponent>,
    mut animation_players: Query<&mut AnimationPlayer>,
    animation_player_links: Query<&AnimationEntityLink>,
    animations: Res<Animations>,
    assets_animation_clips: Res<Assets<AnimationClip>>,
    mut transforms: Query<&mut Transform>,
) {
    for (
        _,
        combatant_id_component,
        skeleton_entity,
        hitbox_radius,
        home_location,
        mut animation_manager_component,
        mut action_result_manager,
        equipment_component,
    ) in &mut combatants
    {
        let species = species_query
            .get(skeleton_entity.0)
            .expect("skeleton to have a species");

        // process all active actions
        let now = Date::new_0().get_time() as u64;
        for (model_action, model_action_progress_tracker) in
            animation_manager_component.active_model_actions.clone()
        {
            let elapsed = now - model_action_progress_tracker.time_started;
            let transition_started = model_action_progress_tracker.transition_started;
            let mut skeleton_entity_transform = transforms
                .get_mut(skeleton_entity.0)
                .expect("the skeleton entity to have a transform");
            match model_action {
                CombatantModelActions::ApproachMeleeTarget => {
                    combatant_approaching_melee_target_processor(
                        &mut skeleton_entity_transform,
                        skeleton_entity.0,
                        &species.0,
                        &equipment_component.0,
                        &mut animation_manager_component,
                        &home_location.0,
                        elapsed,
                        &animations,
                        &mut animation_players,
                        &animation_player_links,
                        transition_started,
                    )
                }
                CombatantModelActions::ReturnHome => todo!(),
                CombatantModelActions::Recenter => todo!(),
                CombatantModelActions::TurnToFaceTarget => todo!(),
                CombatantModelActions::AttackMeleeMainHand => todo!(),
                CombatantModelActions::AttackMeleeOffHand => todo!(),
                CombatantModelActions::HitRecovery => todo!(),
                CombatantModelActions::Death => todo!(),
                CombatantModelActions::Idle => todo!(),
                CombatantModelActions::Evade => todo!(),
            }
        }
    }
}
