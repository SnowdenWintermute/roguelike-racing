use self::model_actions::get_animation_name_from_model_action;
use self::model_actions::CombatantModelActionProgressTracker;
use self::model_actions::CombatantModelActions;
use crate::bevy_app::utils::link_animations::AnimationEntityLink;
use crate::frontend_common::CombatantSpecies;
use bevy::math::u64;
use bevy::prelude::*;
use common::items::equipment::EquipmentSlots;
use common::items::Item;
use js_sys::Date;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::time::Duration;

use super::Animations;
mod approaching_melee_target;
mod attack_melee_main_hand;
mod hit_recovery;
pub mod model_actions;
pub mod process_active_model_actions;
mod start_idle_animation;
pub mod start_new_model_actions_or_idle;

pub type Timestamp = u64;

#[derive(Debug, Clone)]
pub enum FloatingTextType {
    Number(u16),
    Text(String),
}

#[derive(Debug, Clone)]
pub struct FloatingText {
    pub value: FloatingTextType,
    pub home_location: Transform,
    pub destination: Transform,
    pub entity: Entity,
    pub time_started: u64,
    pub color_option: Option<Vec3>,
}

#[derive(Component, Default)]
pub struct TransformManager {
    pub destination: Option<Transform>,
    pub last_location: Option<Transform>,
    pub target_rotation: Option<Quat>,
    pub last_rotation: Option<Quat>,
}

#[derive(Component, Default)]
pub struct ActiveModelActions(HashMap<CombatantModelActions, CombatantModelActionProgressTracker>);
#[derive(Component, Default)]
pub struct ModelActionQueue(pub VecDeque<CombatantModelActions>);
#[derive(Component, Default)]
pub struct FloatingTextComponent(Vec<FloatingText>);

impl ModelActionQueue {
    /// takes the next model action in the queue and adds it to the list of active model actions
    /// marks the start time as now an begins any associated animation
    pub fn start_next_model_action(
        &mut self,
        active_model_actions: &mut ActiveModelActions,
        animation_player_links: &Query<&AnimationEntityLink>,
        animation_players: &mut Query<&mut AnimationPlayer>,
        animations: &Res<Animations>,
        skeleton_entity: Entity,
        combatant_species: &CombatantSpecies,
        equipment: &HashMap<EquipmentSlots, Item>,
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

            if let Some(animation_name) =
                get_animation_name_from_model_action(&combatant_species, &model_action, &equipment)
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
                    .start_with_transition(
                        animation_handle.clone(),
                        Duration::from_millis(transition_duration_ms),
                    )
                    .repeat();
            };
        }
    }
}
