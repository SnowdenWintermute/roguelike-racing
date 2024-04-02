pub mod draw_direction_ray_gizmos;
pub mod move_entities_toward_destinations;
pub mod process_active_animation_states;
mod process_combatant_approaching_melee_target;
mod process_combatant_hit_recovery;
mod process_combatant_recentering;
mod process_combatant_returning_to_home_position;
mod process_combatant_swinging_weapons;
mod rotate_transform_toward_target;
pub mod start_combatant_hit_recoveries;
mod translate_transform_toward_target;
use super::animation_manager_component::ActionSequenceStates;
use super::animation_manager_component::AnimationManagerComponent;
use super::spawn_combatant::CombatantIdComponent;
use super::spawn_combatant::CombatantSpeciesComponent;
use super::spawn_combatant::HitboxRadius;
use super::spawn_combatant::MainSkeletonEntity;
use super::Animations;
use super::CombatantsById;
use super::CombatantsExecutingAttacks;
use crate::bevy_app::utils::link_animations::AnimationEntityLink;
use crate::comm_channels::StartAttackSequenceEvent;
use crate::frontend_common::animation_names::AnimationType;
use crate::frontend_common::animation_names::CombatantAnimations;
use crate::frontend_common::AttackCommand;
use crate::frontend_common::CombatantSpecies;
use bevy::prelude::*;
use js_sys::Date;
use std::time::Duration;

pub fn handle_attack_sequence_start_requests(
    combatants_by_id: Res<CombatantsById>,
    mut combatants: Query<(
        &CombatantIdComponent,
        &MainSkeletonEntity,
        &mut AnimationManagerComponent,
        &HitboxRadius,
    )>,
    species_query: Query<&CombatantSpeciesComponent>,
    animation_player_links: Query<&AnimationEntityLink>,
    mut animation_players: Query<&mut AnimationPlayer>,
    transforms: Query<&mut Transform>,
    mut attack_sequence_commands_event_reader: EventReader<StartAttackSequenceEvent>,
    animations: Res<Animations>,
    mut combatants_with_active_action_states: ResMut<CombatantsExecutingAttacks>,
) {
    for event in attack_sequence_commands_event_reader.read() {
        let AttackCommand {
            combatant_id,
            target_id,
        } = event.0;

        // get locations of combatant and target
        let target_entity = combatants_by_id
            .0
            .get(&target_id)
            .expect("to have the entity");
        let (_, target_skeleton_entity, _, target_hitbox_radius) = combatants
            .get(*target_entity)
            .expect("to have the combatant");

        let cloned_target_hitbox_radius = target_hitbox_radius.clone();
        let target_transform = transforms
            .get(target_skeleton_entity.0)
            .expect("to have the transform")
            .clone();
        let combatant_entity = combatants_by_id
            .0
            .get(&combatant_id)
            .expect("to have the entity");

        let (_, combatant_skeleton_entity, mut combatant_animation_manager, _) = combatants
            .get_mut(*combatant_entity)
            .expect("to have the combatant");

        let combatant_species = species_query
            .get(combatant_skeleton_entity.0)
            .expect("to have a species on the skeleton");

        if combatant_animation_manager.active_states.len() > 0 {
            continue;
        }

        let combatant_transform = transforms
            .get(combatant_skeleton_entity.0)
            .expect("to have the transform")
            .clone();

        // Calculate destination
        let direction =
            (combatant_transform.translation - target_transform.translation).normalize();
        let destination = target_transform.translation + direction * cloned_target_hitbox_radius.0;
        combatant_animation_manager.destination = Some(Transform::from_xyz(
            destination[0],
            destination[1],
            destination[2],
        ));

        let up = *combatant_transform.up().clone();
        combatant_animation_manager.target_rotation = Some(
            combatant_transform
                .looking_at(
                    combatant_animation_manager
                        .destination
                        .expect("declared above")
                        .translation,
                    up,
                )
                .rotation,
        );
        let time_started = Date::new_0().get_time() as u64;
        combatant_animation_manager
            .active_states
            .insert(ActionSequenceStates::ApproachingTarget, Some(time_started));
        combatant_animation_manager.current_targets = Some(Vec::from([event.0.target_id]));

        combatants_with_active_action_states.0.insert(combatant_id);

        // Start animation
        let animation_player_link = animation_player_links
            .get(combatant_skeleton_entity.0)
            .expect("to have linked the skeleton to it's animation player");
        let mut animation_player = animation_players
            .get_mut(animation_player_link.0)
            .expect("to have a valid animation player entity in the link");

        let anim_name = combatant_species.0.animation_name(AnimationType::Run);

        let animation_handle = animations
            .0
            .get(&anim_name)
            .expect("to have a run animation");
        animation_player
            .play_with_transition(animation_handle.clone(), Duration::from_millis(500))
            .repeat();
    }
}
