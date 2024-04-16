use super::handle_combat_turn_results::combatant_model_actions::get_animation_name_from_model_action;
use super::handle_combat_turn_results::combatant_model_actions::CombatantModelActions;
use super::Animations;
use crate::bevy_app::modular_character_plugin::handle_combat_turn_results::combatant_model_actions::CombatantModelActionProgressTracker;
use crate::bevy_app::utils::link_animations::AnimationEntityLink;
use crate::frontend_common::CombatantSpecies;
use super::CombatantId;
use bevy::math::u64;
use bevy::prelude::*;
use js_sys::Date;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::time::Duration;

pub type Timestamp = u64;

#[derive(Debug, Eq, Hash, PartialEq, Clone)]
pub enum ActionSequenceStates {
    ApproachingTarget,
    Swinging,
    Returning,
    Recentering,
    HitRecovery,
}

#[derive(Debug, Clone)]
pub struct HpChangeNumber {
    pub value: u16,
    pub home_location: Transform,
    pub destination: Transform,
    pub entity: Entity,
    pub time_started: u64,
}

#[derive(Component, Default)]
pub struct AnimationManagerComponent {
    pub model_action_queue: VecDeque<CombatantModelActions>,
    pub active_model_actions: HashMap<CombatantModelActions, CombatantModelActionProgressTracker>,
    pub destination: Option<Transform>,
    pub last_location: Option<Transform>,
    pub target_rotation: Option<Quat>,
    pub last_rotation: Option<Quat>,
    pub current_targets: Option<Vec<CombatantId>>,
    pub hp_change_numbers: Vec<HpChangeNumber>,
}

impl AnimationManagerComponent {
    /// takes the next model action in the queue and adds it to the list of active model actions
    /// marks the start time as now an begins any associated animation
    pub fn start_next_model_action(
        &mut self,
        animation_player_links: &Query<&AnimationEntityLink>,
        animation_players: &mut Query<&mut AnimationPlayer>,
        animations: &Res<Animations>,
        skeleton_entity: Entity,
        combatant_species: &CombatantSpecies,
        transition_duration_ms: u64,
    ) {
        if let Some(model_action) = self.model_action_queue.pop_front() {
            self.active_model_actions.insert(
                model_action.clone(),
                CombatantModelActionProgressTracker {
                    time_started: Date::new_0().get_time() as u64,
                    transition_started: false,
                },
            );
            // start animation if any

            if let Some(animation_name) =
                get_animation_name_from_model_action(&combatant_species, &model_action)
            {
                let animation_player_link = animation_player_links
                    .get(skeleton_entity)
                    .expect("to have linked the skeleton to it's animation player");
                let mut animation_player = animation_players
                    .get_mut(animation_player_link.0)
                    .expect("to have a valid animation player entity in the link");
                let animation_handle = animations
                    .0
                    .get(&animation_name)
                    .expect("to be looking up a valid animation");
                animation_player
                    .play_with_transition(
                        animation_handle.clone(),
                        Duration::from_millis(transition_duration_ms),
                    )
                    .repeat();
            };
        }
    }
}
