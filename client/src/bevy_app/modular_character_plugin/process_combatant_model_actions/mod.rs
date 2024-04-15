mod approaching_melee_target;
use super::animation_manager_component::AnimationManagerComponent;
use super::handle_combat_turn_results::combatant_model_actions::CombatantModelActionProgressTracker;
use super::handle_combat_turn_results::combatant_model_actions::CombatantModelActions;
use super::spawn_combatant::CombatantActionResultsManagerComponent;
use super::spawn_combatant::CombatantIdComponent;
use super::spawn_combatant::CombatantSpeciesComponent;
use super::spawn_combatant::HitboxRadius;
use super::spawn_combatant::MainSkeletonEntity;
use super::Animations;
use super::CombatantsById;
use crate::bevy_app::utils::link_animations::AnimationEntityLink;
use bevy::prelude::*;
use js_sys::Date;

pub fn process_combatant_model_actions(
    combatants_by_id: Res<CombatantsById>,
    mut combatants: Query<(
        &CombatantIdComponent,
        &CombatantSpeciesComponent,
        &MainSkeletonEntity,
        &HitboxRadius,
        &mut AnimationManagerComponent,
        &mut CombatantActionResultsManagerComponent,
    )>,
    species_query: Query<&CombatantSpeciesComponent>,
    animation_player_links: Query<&AnimationEntityLink>,
    mut animation_players: Query<&mut AnimationPlayer>,
    animations: Res<Animations>,
    transforms: Query<&mut Transform>,
) {
    for (
        id_component,
        combatant_species_component,
        skeleton_entity,
        hitbox_radius,
        mut animation_manager_component,
        action_result_manager,
    ) in &mut combatants
    {
        // if no active actions, take the next one
        if animation_manager_component.active_model_actions.len() < 1 {
            animation_manager_component.start_next_model_action(
                &animation_player_links,
                &mut animation_players,
                &animations,
                skeleton_entity.0,
                combatant_species_component.0.clone(),
            );
        }
        // process all active actions
        for model_action in &animation_manager_component.active_model_actions {
            process_model_action(&model_action.0, &model_action.1);
        }
    }
}

pub fn process_model_action(
    model_action: &CombatantModelActions,
    model_action_progress_tracker: &CombatantModelActionProgressTracker,
) {
    match model_action {
        CombatantModelActions::ApproachMeleeTarget => todo!(),
        CombatantModelActions::ReturnHome => todo!(),
        CombatantModelActions::Recenter => todo!(),
        CombatantModelActions::TurnToFaceTarget => todo!(),
        CombatantModelActions::AttackMeleeMainHand => todo!(),
        CombatantModelActions::AttackMeleeOffHand => todo!(),
    }
}
