pub mod ability_handlers;
pub mod battle;
mod turn_order;
use self::battle::BattleGroup;
use crate::combatants::abilities::{AbilityTarget, CombatantAbilityNames};
use crate::items::consumables::ConsumableTypes;
use crate::items::Item;
use crate::primatives::GainedOrLost;
use crate::status_effects::StatusEffects;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub enum CombatAction {
    AbilityUsed(CombatantAbilityNames),
    ItemUsed(Item),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IdAndValue(u32, i16);

#[derive(Debug, Serialize, Deserialize)]
pub struct CombatActionResult {
    user_id: u32,
    // Used to select the animation played and to remove consumed items from inventory if
    // appropriate
    action: CombatAction,
    // Used to modify the animation and direction character faces when
    // using ability
    targets: AbilityTarget,
    // used to display floating text and reduce/add to the value
    hp_changes_by_entity_id: Option<Vec<IdAndValue>>,
    mp_changes_by_entity_id: Option<Vec<IdAndValue>>,
    misses_by_entity_id: Option<Vec<u32>>,
    resists_by_entity_id: Option<Vec<u32>>,
    is_crit: bool,
    // used to display floating +- effect icons and add/remove the effects to entities
    status_effect_changes_by_entity_id: Option<HashMap<u32, Vec<(StatusEffects, GainedOrLost)>>>,
}

#[derive(Debug)]
pub enum CombatActionEffect {
    AbilityUsed(CombatantAbilityNames, Vec<u32>),
    ConsumableUsed(ConsumableTypes, Vec<u32>),
    CurrentHpChange(i16, u32),
    CurrentMpChange(i16, u32),
    StatusEffectGained(StatusEffects, u32),
    StatusEffectLost(StatusEffects, u32),
    CombatantDeath(u32),
    EndTurn,
}
