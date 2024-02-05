pub mod get_combatant_ability_attributes;
use serde::Deserialize;
use serde::Serialize;
use std::fmt::Display;

use crate::primatives::Range;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CombatantAbility {
    pub ability_name: CombatantAbilityNames,
    pub level: u8,
    pub base_values: Option<Range<u8>>,
}

impl CombatantAbility {
    pub fn new(ability_name: CombatantAbilityNames, base_values: Option<Range<u8>>) -> Self {
        CombatantAbility {
            ability_name,
            ..Default::default()
        }
    }
}

impl Default for CombatantAbility {
    fn default() -> CombatantAbility {
        CombatantAbility {
            ability_name: CombatantAbilityNames::Attack,
            level: 0,
            base_values: None,
        }
    }
}

#[derive(Debug, Clone, Hash, Eq, PartialEq, Serialize, Deserialize, PartialOrd, Ord)]
pub enum CombatantAbilityNames {
    Attack,
    ArmorBreak,
    HeatLance,
    Fire,
    RainStorm,
    Heal,
}

impl Display for CombatantAbilityNames {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CombatantAbilityNames::Attack => write!(f, "Attack"),
            CombatantAbilityNames::HeatLance => write!(f, "Heat Lance"),
            CombatantAbilityNames::ArmorBreak => write!(f, "Armor Break"),
            CombatantAbilityNames::Heal => write!(f, "Heal"),
            CombatantAbilityNames::Fire => write!(f, "Fire"),
            CombatantAbilityNames::RainStorm => write!(f, "Rain Storm"),
        }
    }
}

impl CombatantAbility {
    /// Create an instance of an ability with the default values
    pub fn create_by_name(name: &CombatantAbilityNames) -> CombatantAbility {
        match name {
            CombatantAbilityNames::Attack => {
                let mut ability = CombatantAbility::new(CombatantAbilityNames::Attack, None);
                ability.level = 1;
                ability
            }
            name => CombatantAbility::new(name.clone(), None),
        }
    }
}
