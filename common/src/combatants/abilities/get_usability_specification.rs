use super::CombatantAbilityNames;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum TargetingScheme {
    Single,
    Area,
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

impl CombatantAbilityNames {
    pub fn get_targeting_and_usability_specification(&self) -> TargetCategories {
        match self {
            CombatantAbilityNames::Attack => todo!(),
            CombatantAbilityNames::ArmorBreak => todo!(),
            CombatantAbilityNames::HeatLance => todo!(),
            CombatantAbilityNames::Fire => todo!(),
            CombatantAbilityNames::Heal => todo!(),
        }
    }
}
