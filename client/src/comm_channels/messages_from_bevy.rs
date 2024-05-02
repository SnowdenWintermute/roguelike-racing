use crate::frontend_common::PartsByName;
use bevy::math::Vec3;
use common::combat::ActionResult;
use common::primatives::EntityId;
use std::collections::HashSet;

#[derive(Clone, Debug, PartialEq, Default)]
pub struct CameraPosition {
    pub focus: Vec3,
    pub alpha: Option<f32>,
    pub beta: Option<f32>,
    pub radius: Option<f32>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct CombatantIdWithValue {
    pub combatant_id: EntityId,
    pub value: i16,
}

// BEVY MESSAGES
#[derive(Debug, Clone, PartialEq)]
pub enum MessageFromBevy {
    PartNames(PartsByName),
    AnimationsAvailable(HashSet<String>),
    CombatantSpawned(EntityId),
    AssetsLoaded,
    CameraPosition(CameraPosition),
    FinishedProcessingTurnResult(EntityId),
    FinishedAnimating(EntityId),
    StartedProcessingActionResults(EntityId),
    ApplyActionResult(ActionResult),
}
