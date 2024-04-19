use super::approaching_melee_target::combatant_approaching_melee_target_processor;
use super::attack_melee_main_hand::attacking_with_melee_main_hand_processor;
use super::hit_recovery::hit_recovery_processor;
use super::model_actions::CombatantModelActions;
use super::ActiveModelActions;
use super::ModelActionQueue;
use super::TransformManager;
use crate::bevy_app::asset_loader_plugin::MyAssets;
use crate::bevy_app::modular_character_plugin::spawn_combatant::CombatantActionResultsManagerComponent;
use crate::bevy_app::modular_character_plugin::spawn_combatant::CombatantEquipment;
use crate::bevy_app::modular_character_plugin::spawn_combatant::CombatantIdComponent;
use crate::bevy_app::modular_character_plugin::spawn_combatant::CombatantMainArmatureEntityLink;
use crate::bevy_app::modular_character_plugin::spawn_combatant::CombatantSpeciesComponent;
use crate::bevy_app::modular_character_plugin::spawn_combatant::HitboxRadius;
use crate::bevy_app::modular_character_plugin::spawn_combatant::MainSkeletonBonesAndArmature;
use crate::bevy_app::modular_character_plugin::spawn_combatant::MainSkeletonEntity;
use crate::bevy_app::modular_character_plugin::update_scene_aabbs::SceneAabb;
use crate::bevy_app::modular_character_plugin::Animations;
use crate::bevy_app::modular_character_plugin::CombatantsById;
use crate::bevy_app::modular_character_plugin::HomeLocation;
use crate::bevy_app::modular_character_plugin::StartNextModelActionEvent;
use crate::bevy_app::utils::link_animations::AnimationEntityLink;
use crate::comm_channels::BevyTransmitter;
use bevy::ecs::system::SystemParam;
use bevy::prelude::*;
use js_sys::Date;
use std::collections::HashMap;

// check for new active_model_actions
// if newly activated, start the associated animation
// check percent completed and activate next action if beyond threshold
// if done, remove from active_model_actions

#[derive(SystemParam)]
pub struct ModelActionSystemParams<'w, 's> {
    pub animations: Res<'w, Animations>,
    pub animation_players: Query<'w, 's, &'static mut AnimationPlayer>,
    pub animation_player_links: Query<'w, 's, &'static AnimationEntityLink>,
    pub assets_animation_clips: Res<'w, Assets<AnimationClip>>,
    pub transforms: Query<'w, 's, &'static mut Transform>,
    pub combatants_query: Query<
        'w,
        's,
        (
            Entity,
            &'static MainSkeletonBonesAndArmature, // to ensure skeleton is assigned already
            &'static CombatantMainArmatureEntityLink,
            &'static CombatantIdComponent,
            &'static MainSkeletonEntity,
            &'static HitboxRadius,
            &'static HomeLocation,
            &'static CombatantSpeciesComponent,
            &'static mut CombatantActionResultsManagerComponent,
            &'static CombatantEquipment,
            &'static mut TransformManager,
            &'static mut ModelActionQueue,
            &'static mut ActiveModelActions,
        ),
    >,
}

pub fn process_active_model_actions(
    mut commands: Commands,
    mut model_action_params: ModelActionSystemParams,
    mut start_next_model_action_event_writer: EventWriter<StartNextModelActionEvent>,
    combatants_by_id: Res<CombatantsById>,
    asset_pack: Res<MyAssets>,
    scenes_with_aabbs: Query<&SceneAabb>,
    bevy_transmitter: Res<BevyTransmitter>,
) {
    let mut actions_to_add_by_entity: HashMap<Entity, Vec<CombatantModelActions>> = HashMap::new();
    let mut entities_and_active_model_actions = Vec::new();
    for (entity, .., active_model_actions) in &model_action_params.combatants_query {
        entities_and_active_model_actions.push((entity, active_model_actions.0.clone()));
    }

    for (entity, active_model_actions) in entities_and_active_model_actions.into_iter() {
        let mut active_actions_to_remove: Vec<CombatantModelActions> = Vec::new();
        for (model_action, model_action_progress_tracker) in active_model_actions {
            let now = Date::new_0().get_time() as u64;
            let elapsed = now - model_action_progress_tracker.time_started;
            let transition_started = model_action_progress_tracker.transition_started;
            match model_action {
                CombatantModelActions::ApproachMeleeTarget => {
                    combatant_approaching_melee_target_processor(
                        entity,
                        elapsed,
                        transition_started,
                        &mut model_action_params,
                        &mut active_actions_to_remove,
                        &mut start_next_model_action_event_writer,
                    )
                }
                CombatantModelActions::ReturnHome => todo!(),
                CombatantModelActions::Recenter => todo!(),
                CombatantModelActions::TurnToFaceTarget => todo!(),
                CombatantModelActions::AttackMeleeMainHand => {
                    // attacking_with_melee_main_hand_processor(
                    //     &mut commands,
                    //     entity,
                    //     &mut skeleton_entity_transform,
                    //     skeleton_entity.0,
                    //     &species.0,
                    //     &equipment_component.0,
                    //     &mut action_result_manager,
                    //     &home_location.0,
                    //     elapsed,
                    //     &combatants_by_id,
                    //     &animations,
                    //     &mut animation_managers,
                    //     &mut animation_players,
                    //     &animation_player_links,
                    //     &assets_animation_clips,
                    //     &asset_pack,
                    //     &scenes_with_aabbs,
                    //     &main_armature_links,
                    //     &main_skeleton_links,
                    //     transition_started,
                    //     &bevy_transmitter,
                    // )
                }
                CombatantModelActions::AttackMeleeOffHand => todo!(),
                CombatantModelActions::HitRecovery => todo!(),
                // hit_recovery_processor(
                // &mut commands,
                // entity,
                // &skeleton_entity_transform.clone(),
                // skeleton_entity.0,
                // &species.0,
                // &mut animation_managers,
                // &mut transforms,
                // elapsed,
                // now,
                // &animations,
                // &mut animation_players,
                // &animation_player_links,
                // &assets_animation_clips,
                // transition_started,
                // ),
                CombatantModelActions::Death => todo!(),
                CombatantModelActions::Idle => todo!(),
                CombatantModelActions::Evade => info!("evaded"),
            }
        }
    }
}
