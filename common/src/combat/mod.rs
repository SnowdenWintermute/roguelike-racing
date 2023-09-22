use crate::character::abilities::{
    CombatantAbilities, CombatantAbility, CombatantAbilityTargetingScheme,
};
use crate::character::combatant_properties::CombatantProperties;
use crate::errors::AppError;
use crate::status_effects::StatusEffects;
use std::vec;

use crate::items::{ConsumableTypes, Item};

#[derive(Debug)]
pub enum CombatAction<'a> {
    UseCombatantAbility(&'a CombatantAbility),
    UseItem(&'a Item),
}

#[derive(Debug)]
pub enum CombatActionTypes {
    AbilityUsed(CombatantAbilities),
    ItemUsed(ConsumableTypes),
}

#[derive(Debug)]
pub enum CombatActionEffect {
    CurrentHpChange(i16),
    CurrentMpChange(i16),
    StatusEffectGained(StatusEffects),
    StatusEffectLost(StatusEffects),
    EndTurn,
}

#[derive(Debug)]
pub struct CombatEvent {
    pub action_type: CombatActionTypes,
    pub perpetrator_id: u32,
    pub target_id: u32,
    pub action_effects: Vec<CombatActionEffect>,
}

impl CombatantProperties {
    pub fn perform_combat_action(
        &self,
        perpetrator_id: u32,
        target_id: u32,
        targeting_scheme: CombatantAbilityTargetingScheme,
        combat_action: CombatAction,
        opponent_combatant_properties: CombatantProperties,
    ) -> Result<Vec<CombatEvent>, AppError> {
        let mut combat_events: Vec<CombatEvent> = vec![];
        //
        Ok(combat_events)
    }
}
