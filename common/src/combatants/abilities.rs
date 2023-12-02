use std::fmt::Display;

use crate::combatants::CombatantClass;
use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum TargetingScheme {
    Single,
    Area,
    TargetAndAdjacent,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ValidTargets {
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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CombatantAbility {
    pub ability_type: CombatantAbilityNames,
    pub class: Option<CombatantClass>,
    pub level: u8,
    pub mana_cost: u8,
    pub mana_cost_level_multiplier: u8,
    pub shard_cost: u8,
    pub requires_combat_turn: bool,
    pub usable_context: AbilityUsableContext,
    pub targeting_schemes: Vec<TargetingScheme>,
    pub valid_targets: ValidTargets,
}

impl Default for CombatantAbility {
    fn default() -> CombatantAbility {
        CombatantAbility {
            ability_type: CombatantAbilityNames::Attack,
            class: None,
            level: 0,
            mana_cost: 0,
            mana_cost_level_multiplier: 1,
            shard_cost: 0,
            requires_combat_turn: true,
            usable_context: AbilityUsableContext::InCombat,
            targeting_schemes: vec![TargetingScheme::Single],
            valid_targets: ValidTargets::Opponent,
        }
    }
}

#[derive(Debug, Clone, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub enum CombatantAbilityNames {
    Attack,
    HeatLance,
    ArmorBreak,
    ShootArrow,
    Heal,
}

impl Display for CombatantAbilityNames {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CombatantAbilityNames::Attack => write!(f, "Attack"),
            CombatantAbilityNames::HeatLance => write!(f, "Heat Lance"),
            CombatantAbilityNames::ArmorBreak => write!(f, "Armor Break"),
            CombatantAbilityNames::ShootArrow => write!(f, "Shoot Arrow"),
            CombatantAbilityNames::Heal => write!(f, "Heal"),
        }
    }
}

impl CombatantAbility {
    pub fn new(name: &CombatantAbilityNames) -> CombatantAbility {
        match name {
            CombatantAbilityNames::Attack => CombatantAbility {
                ability_type: CombatantAbilityNames::Attack,
                class: None,
                level: 1,
                ..Default::default()
            },
            CombatantAbilityNames::HeatLance => CombatantAbility {
                ability_type: CombatantAbilityNames::HeatLance,
                class: Some(CombatantClass::Mage),
                mana_cost: 1,
                ..Default::default()
            },
            CombatantAbilityNames::ArmorBreak => CombatantAbility {
                ability_type: CombatantAbilityNames::ArmorBreak,
                class: Some(CombatantClass::Warrior),
                mana_cost: 1,
                ..Default::default()
            },
            CombatantAbilityNames::ShootArrow => CombatantAbility {
                ability_type: CombatantAbilityNames::ShootArrow,
                class: Some(CombatantClass::Rogue),
                shard_cost: 1,
                ..Default::default()
            },
            CombatantAbilityNames::Heal => CombatantAbility {
                ability_type: CombatantAbilityNames::Heal,
                class: Some(CombatantClass::Warrior),
                mana_cost: 1,
                usable_context: AbilityUsableContext::OutOfCombat,
                ..Default::default()
            },
        }
    }
}
