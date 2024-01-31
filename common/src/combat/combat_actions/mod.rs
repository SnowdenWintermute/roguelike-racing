pub mod filter_possible_target_ids_by_prohibited_combatant_states;
pub mod get_default_targets;
pub mod get_next_or_previous_targets;
pub mod targets_are_valid;
pub mod targets_by_saved_preference_or_default;
pub mod validate_use;
use crate::combatants::abilities::CombatantAbilityNames;
use serde::Deserialize;
use serde::Serialize;
use std::fmt::Display;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum FriendOrFoe {
    Friendly,
    Hostile,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum CombatActionTarget {
    Single(u32),
    Group(FriendOrFoe),
    All,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum TargetingScheme {
    Single,
    Area,
    All,
}

impl Display for TargetingScheme {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let to_write = match self {
            TargetingScheme::Single => "Single",
            TargetingScheme::Area => "Area",
            TargetingScheme::All => "All",
        };
        write!(f, "{}", to_write)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum TargetCategories {
    Opponent,
    User,
    Friendly,
    Any,
}

impl Display for TargetCategories {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let to_write = match self {
            TargetCategories::Opponent => "Enemy",
            TargetCategories::User => "Self",
            TargetCategories::Friendly => "Friendly",
            TargetCategories::Any => "Any",
        };
        write!(f, "{}", to_write)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ProhibitedTargetCombatantStates {
    Dead,
    Alive,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum AbilityUsableContext {
    All,
    InCombat,
    OutOfCombat,
}

impl Display for AbilityUsableContext {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let to_write = match self {
            AbilityUsableContext::All => "any time",
            AbilityUsableContext::InCombat => "in combat",
            AbilityUsableContext::OutOfCombat => "out of combat",
        };
        write!(f, "{}", to_write)
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum CombatAction {
    AbilityUsed(CombatantAbilityNames),
    ConsumableUsed(u32),
}

pub struct CombatActionProperties {
    pub targeting_schemes: Vec<TargetingScheme>,
    pub valid_target_categories: TargetCategories,
    pub usability_context: AbilityUsableContext,
    pub prohibited_target_combatant_states: Option<Vec<ProhibitedTargetCombatantStates>>,
    pub requires_combat_turn: bool,
}

impl Default for CombatActionProperties {
    fn default() -> Self {
        CombatActionProperties {
            targeting_schemes: vec![TargetingScheme::Single],
            valid_target_categories: TargetCategories::Opponent,
            usability_context: AbilityUsableContext::InCombat,
            prohibited_target_combatant_states: None,
            requires_combat_turn: true,
        }
    }
}

impl Display for CombatAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        let to_write = match self {
            CombatAction::AbilityUsed(ability_name) => format!("ability {ability_name}"),
            CombatAction::ConsumableUsed(consumable_id) => {
                format!("consumable used {consumable_id}")
            }
        };
        write!(f, "{to_write}")
    }
}
