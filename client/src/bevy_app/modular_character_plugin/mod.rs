use self::assign_skeleton_bones_to_combatants::assign_skeleton_bones_to_combatants;
use self::attack_sequence::draw_direction_ray_gizmos::draw_directional_gizmos;
use self::attack_sequence::handle_attack_sequence_start_requests;
use self::attack_sequence::process_active_animation_states::process_active_animation_states;
use self::attack_sequence::start_combatant_hit_recoveries::start_combatant_hit_recoveries;
use self::draw_aabbs::draw_aabbs;
use self::handle_animation_change_requests::handle_animation_change_requests;
use self::notify_yew_that_assets_are_loaded::notify_yew_that_assets_are_loaded;
use self::part_change_plugin::PartChangePlugin;
use self::register_animations::register_animations;
use self::run_animations::run_animations;
use self::spawn_combatant::spawn_combatants;
use self::update_scene_aabbs::update_scene_aabbs_on_changed_children;
use super::utils::link_animations::link_animations;
use crate::bevy_app::asset_loader_plugin::AssetLoaderState;
use crate::frontend_common::CombatantSpecies;
use bevy::prelude::*;
use bevy::utils::HashMap;
use bevy::utils::HashSet;
pub mod animation_manager_component;
mod assemble_parts;
mod assign_skeleton_bones_to_combatants;
mod attack_sequence;
mod draw_aabbs;
mod handle_animation_change_requests;
mod notify_yew_that_assets_are_loaded;
pub mod part_change_plugin;
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
pub struct CombatantsExecutingAttacks(HashSet<CombatantId>);

#[derive(Default, Debug, Clone, Component)]
pub struct HomeLocation(pub Transform);

// EVENTS
#[derive(Clone, Debug, Event)]
pub struct HitRecoveryActivationEvent(Vec<(CombatantId, u16)>);

pub struct ModularCharacterPlugin;
impl Plugin for ModularCharacterPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<AttachedPartsReparentedEntities>()
            .init_resource::<CombatantsById>()
            .init_resource::<SkeletonsAwaitingCombatantAssignment>()
            .init_resource::<Animations>()
            .init_resource::<CombatantsExecutingAttacks>()
            .init_resource::<Events<HitRecoveryActivationEvent>>()
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
                    assign_skeleton_bones_to_combatants,
                    link_animations,
                    run_animations,
                    handle_animation_change_requests,
                    // draw_directional_gizmos,
                    handle_attack_sequence_start_requests,
                    process_active_animation_states,
                    start_combatant_hit_recoveries,
                    update_scene_aabbs_on_changed_children,
                )
                    .run_if(in_state(AssetLoaderState::Done)),
            )
            .add_systems(
                OnEnter(AssetLoaderState::Done),
                notify_yew_that_assets_are_loaded,
            );
    }
}
