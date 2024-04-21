use self::assign_skeleton_bones_to_combatants::assign_skeleton_bones_to_combatants;
use self::handle_combat_turn_results::enqueue_model_actions_from_action_results::enqueue_model_actions_from_action_results;
use self::handle_combat_turn_results::start_processing_next_action_results::start_processing_next_action_results;
use self::handle_combat_turn_results::start_processing_next_turn_result::start_processing_next_turn_result_in_queue;
use self::handle_despawn_combatant_model_events::handle_despawn_combatant_model_events;
use self::notify_yew_that_assets_are_loaded::notify_yew_that_assets_are_loaded;
use self::part_change_plugin::PartChangePlugin;
use self::process_combatant_model_actions::handle_new_attack_reaction_events::AttackResult;
use self::process_combatant_model_actions::handle_start_next_model_action_events::handle_start_next_model_action_events;
use self::process_combatant_model_actions::process_active_model_actions::process_active_model_actions;
use self::process_combatant_model_actions::start_new_model_actions_or_idle::start_new_model_actions_or_idle;
use self::process_combatant_model_actions::FloatingTextType;
use self::register_animations::register_animations;
use self::spawn_combatant::spawn_combatants;
use self::update_scene_aabbs::update_scene_aabbs_on_changed_children;
use super::utils::link_animations::link_animations;
use super::BevyAppState;
use crate::bevy_app::asset_loader_plugin::AssetLoaderState;
use crate::comm_channels::DespawnCombatantModelEvent;
use crate::frontend_common::CombatantSpecies;
use bevy::prelude::*;
use common::combat::CombatTurnResult;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
mod assemble_parts;
mod assign_skeleton_bones_to_combatants;
mod attack_sequence;
mod draw_aabbs;
mod handle_combat_turn_results;
pub mod handle_despawn_combatant_model_events;
mod notify_yew_that_assets_are_loaded;
pub mod part_change_plugin;
mod process_combatant_model_actions;
mod register_animations;
mod run_animations;
mod spawn_combatant;
pub mod spawn_scenes;
mod update_scene_aabbs;

pub type CombatantId = u32;

// RESOURCES
#[derive(Resource, Debug, Default)]
pub struct SkeletonsAwaitingCombatantAssignment(
    pub HashMap<CombatantId, (Entity, CombatantSpecies)>,
);
#[derive(Resource, Debug, Default)]
pub struct Animations(pub HashMap<String, Handle<AnimationClip>>);
#[derive(Resource, Debug, Default)]
pub struct CombatantsById(pub HashMap<CombatantId, Entity>);
#[derive(Resource, Default)]
pub struct AttachedPartsReparentedEntities {
    parts_and_entities: HashMap<Entity, Vec<Entity>>,
}

#[derive(Resource, Default)]
pub struct TurnResultsQueue(pub VecDeque<CombatTurnResult>);
#[derive(Resource, Default)]
pub struct CurrentTurnResultProcessing(pub Option<CombatTurnResult>);

#[derive(Default, Debug, Clone, Component)]
pub struct HomeLocation(pub Transform);

// EVENTS
#[derive(Clone, Debug, Event)]
pub struct StartNextModelActionEvent {
    entity: Entity,
    transition_duration_ms: u64,
}
#[derive(Clone, Debug, Event)]
pub struct StartNewAttackReactionEvent {
    entity_id: CombatantId,
    attack_result: AttackResult,
}

pub struct ModularCharacterPlugin;
impl Plugin for ModularCharacterPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<AttachedPartsReparentedEntities>()
            .init_resource::<CombatantsById>()
            .init_resource::<SkeletonsAwaitingCombatantAssignment>()
            .init_resource::<Animations>()
            .init_resource::<Events<StartNextModelActionEvent>>()
            .init_resource::<Events<StartNewAttackReactionEvent>>()
            .init_resource::<Events<DespawnCombatantModelEvent>>()
            .init_resource::<TurnResultsQueue>()
            .init_resource::<CurrentTurnResultProcessing>()
            // .init_::<CombatantsExecutingAttacks>()
            .add_plugins(PartChangePlugin)
            .add_systems(
                OnEnter(AssetLoaderState::RegisteringAnimations),
                (
                    register_animations,
                    // draw_aabbs
                ),
            )
            .add_systems(
                Update,
                (
                    spawn_combatants,
                    handle_despawn_combatant_model_events,
                    assign_skeleton_bones_to_combatants,
                    link_animations,
                    // run_animations,
                    // handle_animation_change_requests,
                    // draw_directional_gizmos,
                    // handle_attack_sequence_start_requests,
                    // process_active_animation_states,
                    // start_combatant_hit_recoveries,
                    update_scene_aabbs_on_changed_children,
                    start_processing_next_turn_result_in_queue,
                    start_processing_next_action_results,
                    enqueue_model_actions_from_action_results,
                    start_new_model_actions_or_idle,
                    process_active_model_actions,
                    handle_start_next_model_action_events,
                )
                    .run_if(in_state(AssetLoaderState::Done))
                    .run_if(in_state(BevyAppState::Running)),
            )
            .add_systems(
                OnEnter(AssetLoaderState::Done),
                notify_yew_that_assets_are_loaded,
            );
    }
}
