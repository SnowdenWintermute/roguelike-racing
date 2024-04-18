use crate::bevy_app::asset_loader_plugin::MyAssets;
use crate::bevy_app::modular_character_plugin::animation_manager_component;
use crate::bevy_app::modular_character_plugin::animation_manager_component::AnimationManagerComponent;
use crate::bevy_app::modular_character_plugin::handle_combat_turn_results::combatant_model_actions::CombatantModelActions;
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
use crate::bevy_app::utils::link_animations::AnimationEntityLink;
use crate::comm_channels::BevyTransmitter;
use crate::frontend_common::CombatantSpecies;
use bevy::prelude::*;
use js_sys::Date;
use super::approaching_melee_target::combatant_approaching_melee_target_processor;
use super::attack_melee_main_hand::attacking_with_melee_main_hand_processor;
use super::hit_recovery::hit_recovery_processor;

pub fn process_active_model_actions(
    mut commands: Commands,
    mut combatants: Query<(
        Entity,
        &MainSkeletonBonesAndArmature, // to ensure skeleton is assigned already
        &CombatantIdComponent,
        &MainSkeletonEntity,
        &HitboxRadius,
        &HomeLocation,
        &mut CombatantActionResultsManagerComponent,
        &CombatantEquipment,
    )>,
    main_armature_links: Query<&CombatantMainArmatureEntityLink>,
    main_skeleton_links: Query<&MainSkeletonEntity>,
    combatants_by_id: Res<CombatantsById>,
    species_query: Query<&CombatantSpeciesComponent>,
    mut animation_managers: Query<&mut AnimationManagerComponent>,
    mut animation_players: Query<&mut AnimationPlayer>,
    animation_player_links: Query<&AnimationEntityLink>,
    animations: Res<Animations>,
    assets_animation_clips: Res<Assets<AnimationClip>>,
    asset_pack: Res<MyAssets>,
    scenes_with_aabbs: Query<&SceneAabb>,
    mut transforms: Query<&mut Transform>,
    bevy_transmitter: Res<BevyTransmitter>,
) {
    for (
        entity,
        _,
        combatant_id_component,
        skeleton_entity,
        hitbox_radius,
        home_location,
        mut action_result_manager,
        equipment_component,
    ) in &mut combatants
    {
        let species = species_query
            .get(skeleton_entity.0)
            .expect("skeleton to have a species");

        // process all active actions
        let now = Date::new_0().get_time() as u64;
        let animation_manager_component = animation_managers
            .get(entity)
            .expect("to have an animation manager");
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
                        entity,
                        &mut skeleton_entity_transform,
                        skeleton_entity.0,
                        &species.0,
                        &equipment_component.0,
                        &mut animation_managers,
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
                CombatantModelActions::AttackMeleeMainHand => {
                    attacking_with_melee_main_hand_processor(
                        &mut commands,
                        entity,
                        &mut skeleton_entity_transform,
                        skeleton_entity.0,
                        &species.0,
                        &equipment_component.0,
                        &mut action_result_manager,
                        &home_location.0,
                        elapsed,
                        &combatants_by_id,
                        &animations,
                        &mut animation_managers,
                        &mut animation_players,
                        &animation_player_links,
                        &assets_animation_clips,
                        &asset_pack,
                        &scenes_with_aabbs,
                        &main_armature_links,
                        &main_skeleton_links,
                        transition_started,
                        &bevy_transmitter,
                    )
                }
                CombatantModelActions::AttackMeleeOffHand => todo!(),
                CombatantModelActions::HitRecovery => hit_recovery_processor(
                    &mut commands,
                    entity,
                    &skeleton_entity_transform.clone(),
                    skeleton_entity.0,
                    &species.0,
                    &mut animation_managers,
                    &mut transforms,
                    elapsed,
                    now,
                    &animations,
                    &mut animation_players,
                    &animation_player_links,
                    &assets_animation_clips,
                    transition_started,
                ),
                CombatantModelActions::Death => todo!(),
                CombatantModelActions::Idle => todo!(),
                CombatantModelActions::Evade => info!("evaded"),
            }
        }
    }
}
