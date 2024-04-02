use super::process_combatant_approaching_melee_target::process_combatant_approaching_melee_target;
use super::process_combatant_hit_recovery::process_combatant_hit_recovery;
use super::process_combatant_recentering::process_combatant_recentering;
use super::process_combatant_returning_to_home_position::process_combatant_returning_to_home_position;
use super::process_combatant_swinging_weapons::process_combatant_swinging_weapons;
use crate::bevy_app::modular_character_plugin::animation_manager_component::ActionSequenceStates;
use crate::bevy_app::modular_character_plugin::animation_manager_component::AnimationManagerComponent;
use crate::bevy_app::modular_character_plugin::spawn_combatant::CombatantSpeciesComponent;
use crate::bevy_app::modular_character_plugin::spawn_combatant::MainSkeletonEntity;
use crate::bevy_app::modular_character_plugin::Animations;
use crate::bevy_app::modular_character_plugin::CombatantsById;
use crate::bevy_app::modular_character_plugin::CombatantsExecutingAttacks;
use crate::bevy_app::modular_character_plugin::HitRecoveryActivationEvent;
use crate::bevy_app::modular_character_plugin::HomeLocation;
use crate::bevy_app::utils::link_animations::AnimationEntityLink;
use crate::frontend_common::CombatantSpecies;
use bevy::prelude::*;
use js_sys::Date;

pub fn process_active_animation_states(
    mut commands: Commands,
    combatants_by_id: Res<CombatantsById>,
    mut combatants: Query<(
        &MainSkeletonEntity,
        &mut AnimationManagerComponent,
        &HomeLocation,
    )>,
    species_query: Query<&CombatantSpeciesComponent>,
    mut transforms: Query<&mut Transform>,
    mut combatants_with_active_action_states: ResMut<CombatantsExecutingAttacks>,
    animation_player_links: Query<&AnimationEntityLink>,
    mut animation_players: Query<&mut AnimationPlayer>,
    animations: Res<Animations>,
    assets_animation_clips: Res<Assets<AnimationClip>>,
    mut hit_recovery_activation_event_writer: EventWriter<HitRecoveryActivationEvent>,
) {
    let cloned_combatants_with_active_action_states =
        combatants_with_active_action_states.0.clone();
    for combatant_id in cloned_combatants_with_active_action_states.iter() {
        let combatant_entity = combatants_by_id
            .0
            .get(combatant_id)
            .expect("to have the combatant");
        let (skeleton_entity, mut animation_manager, home_location) = combatants
            .get_mut(*combatant_entity)
            .expect("to have the combatant");

        let species = species_query
            .get(skeleton_entity.0)
            .expect("a species component on the skeleton");

        if animation_manager.active_states.len() == 0 {
            combatants_with_active_action_states.0.remove(combatant_id);
        }

        let current_time = Date::new_0().get_time() as u64;

        let animation_player_link = animation_player_links
            .get(skeleton_entity.0)
            .expect("to have linked the skeleton to it's animation player");
        let mut animation_player = animation_players
            .get_mut(animation_player_link.0)
            .expect("to have a valid animation player entity in the link");

        let active_states = animation_manager.active_states.clone();

        // info!("active states: {:#?}", active_states);
        for (active_state, time_started_option) in active_states {
            match active_state {
                ActionSequenceStates::ApproachingTarget => {
                    let mut skeleton_entity_transform = transforms
                        .get_mut(skeleton_entity.0)
                        .expect("skeleton to have a tranform");
                    process_combatant_approaching_melee_target(
                        &mut skeleton_entity_transform,
                        &mut animation_manager,
                        &home_location.0,
                        current_time - time_started_option.expect("to have marked the start time"),
                        &mut animation_player,
                        &animations,
                        current_time,
                        &species.0,
                    );
                }
                ActionSequenceStates::Swinging => process_combatant_swinging_weapons(
                    &mut animation_manager,
                    &home_location.0,
                    &mut animation_player,
                    &animations,
                    &assets_animation_clips,
                    current_time,
                    &mut hit_recovery_activation_event_writer,
                    &species.0,
                ),
                ActionSequenceStates::Returning => {
                    let mut skeleton_entity_transform = transforms
                        .get_mut(skeleton_entity.0)
                        .expect("skeleton to have a tranform");
                    process_combatant_returning_to_home_position(
                        &mut skeleton_entity_transform,
                        &mut animation_manager,
                        &home_location.0,
                        current_time - time_started_option.expect("to have marked the start time"),
                        &mut animation_player,
                        &animations,
                        current_time,
                        &species.0,
                    )
                }
                ActionSequenceStates::Recentering => {
                    let mut skeleton_entity_transform = transforms
                        .get_mut(skeleton_entity.0)
                        .expect("skeleton to have a tranform");
                    process_combatant_recentering(
                        &mut skeleton_entity_transform,
                        &mut animation_manager,
                        &home_location.0,
                        current_time - time_started_option.expect("to have marked the start time"),
                    )
                }
                ActionSequenceStates::HitRecovery => {
                    process_combatant_hit_recovery(
                        &mut commands,
                        &mut animation_manager,
                        current_time,
                        &mut animation_player,
                        &animations,
                        &assets_animation_clips,
                        &mut transforms,
                        &species.0,
                    );
                }
            }
        }
    }
}
