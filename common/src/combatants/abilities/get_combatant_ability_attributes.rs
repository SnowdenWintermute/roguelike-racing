use super::CombatantAbilityNames;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum TargetingScheme {
    Single,
    Area,
    All,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum TargetCategories {
    Opponent,
    User,
    Friendly,
    Any,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum AbilityUsableContext {
    All,
    InCombat,
    OutOfCombat,
}

pub struct CombatantAbilityAttributes {
    pub targeting_schemes: Vec<TargetingScheme>,
    pub valid_target_categories: TargetCategories,
    pub usability_context: AbilityUsableContext,
    pub mana_cost: u8,
    pub mana_cost_level_multiplier: u8,
    pub shard_cost: u8,
    pub requires_combat_turn: bool,
}

impl Default for CombatantAbilityAttributes {
    fn default() -> Self {
        CombatantAbilityAttributes {
            targeting_schemes: vec![TargetingScheme::Single],
            valid_target_categories: TargetCategories::Opponent,
            usability_context: AbilityUsableContext::InCombat,
            mana_cost: 1,
            mana_cost_level_multiplier: 1,
            shard_cost: 0,
            requires_combat_turn: true,
        }
    }
}

impl CombatantAbilityNames {
    pub fn get_attributes(&self) -> CombatantAbilityAttributes {
        match self {
            CombatantAbilityNames::Attack => CombatantAbilityAttributes {
                mana_cost: 0,
                valid_target_categories: TargetCategories::Any,
                ..Default::default()
            },
            CombatantAbilityNames::ArmorBreak => CombatantAbilityAttributes {
                ..Default::default()
            },
            CombatantAbilityNames::HeatLance => CombatantAbilityAttributes {
                ..Default::default()
            },
            CombatantAbilityNames::Fire => CombatantAbilityAttributes {
                targeting_schemes: vec![TargetingScheme::Single, TargetingScheme::Area],
                valid_target_categories: TargetCategories::Any,
                ..Default::default()
            },
            CombatantAbilityNames::Heal => CombatantAbilityAttributes {
                targeting_schemes: vec![TargetingScheme::Single, TargetingScheme::Area],
                valid_target_categories: TargetCategories::Friendly,
                ..Default::default()
            },
        }
    }
}
