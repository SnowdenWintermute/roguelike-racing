use super::CombatantId;
use bevy::math::u64;
use bevy::prelude::*;
use std::collections::HashMap;

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
    pub active_states: HashMap<ActionSequenceStates, Option<Timestamp>>,
    pub destination: Option<Transform>,
    pub last_location: Option<Transform>,
    pub target_rotation: Option<Quat>,
    pub last_rotation: Option<Quat>,
    pub current_targets: Option<Vec<CombatantId>>,
    pub hp_change_numbers: Vec<HpChangeNumber>,
}
