use self::model_actions::get_animation_name_from_model_action;
use self::model_actions::CombatantModelActionProgressTracker;
use self::model_actions::CombatantModelActions;
use super::Animations;
use super::HomeLocation;
use super::StartNewFloatingTextEvent;
use crate::bevy_app::bevy_app_consts::COMBATANT_TIME_TO_ROTATE_FULL_CIRCLE;
use crate::bevy_app::bevy_app_consts::COMBATANT_TIME_TO_TRAVEL_ONE_METER;
use crate::bevy_app::bevy_app_consts::UNKNOWN_ANIMATION_DURATION;
use crate::bevy_app::utils::link_animations::AnimationEntityLink;
use crate::frontend_common::CombatantSpecies;
use bevy::math::u64;
use bevy::prelude::*;
use common::combatants::CombatantProperties;
use js_sys::Date;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::time::Duration;
mod animation_only_model_action_processor;
mod approaching_destination;
pub mod combatant_item_event_handlers;
mod end_turn;
pub mod get_percent_animation_completed;
pub mod handle_new_attack_reaction_events;
pub mod handle_start_floating_text_events;
pub mod handle_start_next_model_action_events;
mod model_action_causing_damage_processor;
pub mod model_actions;
pub mod process_active_model_actions;
pub mod process_floating_text;
pub mod process_new_raw_action_results_handler;
pub mod process_next_turn_result_event_handler;
mod recentering;
mod returning_home;
mod set_melee_target_destination_transform_and_rotation;
mod set_non_melee_ability_destination_transform_and_rotation;
mod start_idle_animation;
pub mod start_new_model_actions_or_idle;
mod start_processing_new_action_results;

pub type Timestamp = u64;

#[derive(Component, Default)]
pub struct TransformManager {
    pub destination: Option<Transform>,
    pub last_location: Transform,
    pub target_rotation: Option<Quat>,
    pub distance_last_location_to_destination: f32,
    pub time_to_translate: f32,
    pub angle_last_rotation_to_target: f32,
    pub time_to_rotate: f32,
}

impl TransformManager {
    pub fn new(home_location: HomeLocation) -> Self {
        TransformManager {
            last_location: home_location.0,
            ..Default::default()
        }
    }

    pub fn set_destination(
        &mut self,
        current_transform: Transform,
        destination: Option<Transform>,
    ) {
        self.last_location = current_transform;
        self.destination = destination;

        if let Some(destination) = &self.destination {
            self.distance_last_location_to_destination = self
                .last_location
                .translation
                .distance(destination.translation);
            self.time_to_translate =
                COMBATANT_TIME_TO_TRAVEL_ONE_METER * self.distance_last_location_to_destination;
        }
    }

    pub fn set_target_rotation(&mut self, target_rotation: Option<Quat>) {
        self.target_rotation = target_rotation;

        if let Some(target_rotation) = &self.target_rotation {
            self.angle_last_rotation_to_target = self
                .last_location
                .rotation
                .normalize()
                .angle_between(target_rotation.normalize());
            self.time_to_rotate =
                COMBATANT_TIME_TO_ROTATE_FULL_CIRCLE * self.angle_last_rotation_to_target;
        }
    }
}

pub struct FloatingText {
    value: String,
    home_location: Transform,
    destination: Option<Transform>,
    billboard_entity: Entity,
    time_started: Timestamp,
    color: Vec3,
    time_to_live: u64,
}

#[derive(Component, Default)]
pub struct ActiveModelActions(HashMap<CombatantModelActions, CombatantModelActionProgressTracker>);
#[derive(Component, Default)]
pub struct ModelActionQueue(pub VecDeque<CombatantModelActions>);
#[derive(Component, Default)]
pub struct FloatingTextComponent(HashMap<Entity, FloatingText>);

impl ModelActionQueue {
    /// takes the next model action in the queue and adds it to the list of active model actions
    /// marks the start time as now an begins any associated animation
    pub fn start_next_model_action(
        &mut self,
        active_model_actions: &mut ActiveModelActions,
        animation_player_links: &Query<&AnimationEntityLink>,
        animation_players: &mut Query<&mut AnimationPlayer>,
        start_new_floating_text_event_writer: &mut EventWriter<StartNewFloatingTextEvent>,
        animations: &Res<Animations>,
        combatant_entity: Entity,
        skeleton_entity: Entity,
        combatant_species: &CombatantSpecies,
        combatant_properties: &CombatantProperties,
        transition_duration_ms: u64,
    ) {
        if let Some(model_action) = self.0.pop_front() {
            active_model_actions.0.insert(
                model_action.clone(),
                CombatantModelActionProgressTracker {
                    time_started: Date::new_0().get_time() as u64,
                    transition_started: false,
                },
            );
            // start animation if any
            let should_repeat = match model_action {
                CombatantModelActions::ApproachDestination
                | CombatantModelActions::Recenter
                | CombatantModelActions::ReturnHome => true,
                _ => false,
            };

            let animation_player_link = animation_player_links
                .get(skeleton_entity)
                .expect("to have linked the skeleton to it's animation player");
            let mut animation_player = animation_players
                .get_mut(animation_player_link.0)
                .expect("to have a valid animation player entity in the link");
            if let Some(animation_name) = get_animation_name_from_model_action(
                &combatant_species,
                &model_action,
                &combatant_properties,
            ) {
                animation_player.resume();
                let animation_handle = animations
                    .0
                    .get(&animation_name)
                    .expect("to be looking up a valid animation");
                animation_player.start_with_transition(
                    animation_handle.clone(),
                    Duration::from_millis(transition_duration_ms),
                );
                if should_repeat {
                    animation_player.repeat();
                }
            } else {
                animation_player.pause();
                // show missing animation billboard
                start_new_floating_text_event_writer.send(StartNewFloatingTextEvent {
                    combatant_entity,
                    text: format!("Missing Animation: {:?}", model_action),
                    color: Vec3::from([1.0, 1.0, 1.0]),
                    distance_to_travel: 0.0,
                    time_to_live: UNKNOWN_ANIMATION_DURATION,
                });
            };
        }
    }
}
