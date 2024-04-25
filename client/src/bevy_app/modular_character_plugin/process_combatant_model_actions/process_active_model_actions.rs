use super::animation_only_model_action_processor::animation_only_model_action_processor;
use super::approaching_destination::combatant_approaching_destination_processor;
use super::end_turn::process_combatant_ending_turn;
use super::model_action_causing_damage_processor::model_action_causing_damage_processor;
use super::model_actions::CombatantModelActions;
use super::recentering::combatant_recentering_processor;
use super::returning_home::combatant_returning_to_home_position_home_processor;
use super::ActiveModelActions;
use super::FloatingTextComponent;
use super::ModelActionQueue;
use super::TransformManager;
use crate::bevy_app::modular_character_plugin::spawn_combatant::ActionResultsProcessing;
use crate::bevy_app::modular_character_plugin::spawn_combatant::CombatantIdComponent;
use crate::bevy_app::modular_character_plugin::spawn_combatant::CombatantMainArmatureEntityLink;
use crate::bevy_app::modular_character_plugin::spawn_combatant::CombatantPropertiesComponent;
use crate::bevy_app::modular_character_plugin::spawn_combatant::CombatantSpeciesComponent;
use crate::bevy_app::modular_character_plugin::spawn_combatant::HitboxRadius;
use crate::bevy_app::modular_character_plugin::spawn_combatant::MainSkeletonBonesAndArmature;
use crate::bevy_app::modular_character_plugin::spawn_combatant::MainSkeletonEntity;
use crate::bevy_app::modular_character_plugin::Animations;
use crate::bevy_app::modular_character_plugin::CombatantsById;
use crate::bevy_app::modular_character_plugin::HomeLocation;
use crate::bevy_app::modular_character_plugin::StartNewAttackReactionEvent;
use crate::bevy_app::modular_character_plugin::StartNextModelActionEvent;
use crate::bevy_app::utils::link_animations::AnimationEntityLink;
use crate::comm_channels::BevyTransmitter;
use crate::comm_channels::ProcessNextTurnResultEvent;
use bevy::ecs::query::QueryData;
use bevy::ecs::system::SystemParam;
use bevy::prelude::*;
use js_sys::Date;

#[derive(QueryData)]
#[query_data(mutable)]
pub struct ModelActionCombatantQueryStruct {
    pub entity: Entity,
    pub main_skeleton_bones_and_armature: &'static MainSkeletonBonesAndArmature, // to ensure skeleton is assigned already
    pub armature_link: &'static CombatantMainArmatureEntityLink,
    pub combatant_id_component: &'static CombatantIdComponent,
    pub skeleton_entity: &'static MainSkeletonEntity,
    pub hitbox_radius: &'static HitboxRadius,
    pub home_location: &'static HomeLocation,
    pub combatant_properties_component: &'static mut CombatantPropertiesComponent,
    pub transform_manager: &'static mut TransformManager,
    pub model_action_queue: &'static mut ModelActionQueue,
    pub action_results_processing: &'static mut ActionResultsProcessing,
    pub active_model_actions: &'static mut ActiveModelActions,
    pub floating_text_option: Option<&'static mut FloatingTextComponent>,
}

#[derive(SystemParam)]
pub struct ModelActionSystemParams<'w, 's> {
    pub animations: Res<'w, Animations>,
    pub animation_players: Query<'w, 's, &'static mut AnimationPlayer>,
    pub animation_player_links: Query<'w, 's, &'static AnimationEntityLink>,
    pub assets_animation_clips: Res<'w, Assets<AnimationClip>>,
    pub transforms: Query<'w, 's, &'static mut Transform>,
    pub combatants_by_id: Res<'w, CombatantsById>,
    pub combatants_query: Query<'w, 's, ModelActionCombatantQueryStruct>,
    pub species_query: Query<'w, 's, &'static CombatantSpeciesComponent>,
}

pub fn process_active_model_actions(
    mut model_action_params: ModelActionSystemParams,
    mut start_next_model_action_event_writer: EventWriter<StartNextModelActionEvent>,
    mut start_new_attack_reaction_event_writer: EventWriter<StartNewAttackReactionEvent>,
    mut process_next_turn_result_event_writer: EventWriter<ProcessNextTurnResultEvent>,
    mut bevy_transmitter: ResMut<BevyTransmitter>,
) {
    let mut entities_and_active_model_actions = Vec::new();
    for combatant in &model_action_params.combatants_query {
        entities_and_active_model_actions
            .push((combatant.entity, combatant.active_model_actions.0.clone()));
    }

    for (entity, active_model_actions) in entities_and_active_model_actions.into_iter() {
        for (model_action, model_action_progress_tracker) in active_model_actions {
            let now = Date::new_0().get_time() as u64;
            let elapsed = now - model_action_progress_tracker.time_started;
            let transition_started = model_action_progress_tracker.transition_started;
            match model_action {
                CombatantModelActions::ApproachDestination => {
                    combatant_approaching_destination_processor(
                        entity,
                        elapsed,
                        transition_started,
                        &mut model_action_params,
                        &mut start_next_model_action_event_writer,
                    )
                }
                CombatantModelActions::ReturnHome => {
                    combatant_returning_to_home_position_home_processor(
                        entity,
                        elapsed,
                        transition_started,
                        &mut model_action_params,
                        &mut start_next_model_action_event_writer,
                        &mut bevy_transmitter,
                    )
                }
                CombatantModelActions::Recenter => {
                    combatant_recentering_processor(entity, elapsed, &mut model_action_params)
                }
                CombatantModelActions::TurnToFaceTarget => todo!(),
                CombatantModelActions::AttackMeleeMainHand
                | CombatantModelActions::AttackMeleeOffHand
                | CombatantModelActions::CastSpell
                | CombatantModelActions::UseConsumable => model_action_causing_damage_processor(
                    entity,
                    elapsed,
                    transition_started,
                    &mut model_action_params,
                    &mut start_next_model_action_event_writer,
                    &mut start_new_attack_reaction_event_writer,
                    &model_action,
                    &mut bevy_transmitter,
                ),
                CombatantModelActions::HitRecovery
                | CombatantModelActions::Evade
                | CombatantModelActions::Death => animation_only_model_action_processor(
                    entity,
                    elapsed,
                    &mut model_action_params,
                    &model_action,
                ),
                CombatantModelActions::Idle => (),
                CombatantModelActions::EndTurn => process_combatant_ending_turn(
                    entity,
                    &mut process_next_turn_result_event_writer,
                    &mut model_action_params,
                ),
            }
        }
    }
}
