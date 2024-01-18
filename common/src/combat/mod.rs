pub mod ability_handlers;
pub mod ai_behavior;
pub mod all_combatants_in_group_are_dead;
pub mod battle;
mod get_combatant_by_id;
mod turn_order;
use self::battle::BattleGroup;
use crate::combatants::abilities::AbilityTarget;
use crate::combatants::abilities::CombatantAbilityNames;
use crate::items::consumables::ConsumableTypes;
use crate::items::Item;
use crate::primatives::GainedOrLost;
use crate::status_effects::StatusEffects;
use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt::Display;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum CombatAction {
    AbilityUsed(CombatantAbilityNames),
    ItemUsed(Item),
}

impl Display for CombatAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        let to_write = match self {
            CombatAction::AbilityUsed(ability_name) => format!("ability {ability_name}"),
            CombatAction::ItemUsed(consumable) => {
                format!("item {}", consumable.entity_properties.name)
            }
        };
        write!(f, "{to_write}")
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct CombatTurnResult {
    pub combatant_id: u32,
    pub action_results: Vec<ActionResult>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ActionResult {
    pub user_id: u32,
    // Used to select the animation played and to remove consumed items from inventory if
    // appropriate
    pub action: CombatAction,
    // Used to modify the animation and direction character faces when
    // using ability
    pub targets: AbilityTarget,
    // used to display floating text and reduce/add to the value
    pub hp_changes_by_entity_id: Option<HashMap<u32, i16>>,
    pub mp_changes_by_entity_id: Option<HashMap<u32, i16>>,
    pub misses_by_entity_id: Option<HashSet<u32>>,
    pub resists_by_entity_id: Option<HashSet<u32>>,
    pub is_crit: bool,
    // used to display floating +- effect icons and add/remove the effects to entities
    pub status_effect_changes_by_entity_id:
        Option<HashMap<u32, Vec<(StatusEffects, GainedOrLost)>>>,
    pub ends_turn: bool,
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
