pub mod get_combatant_ability_attributes;
mod get_next_or_previous_targets;
pub mod targets_by_saved_preference_or_default;
use serde::Deserialize;
use serde::Serialize;
use std::fmt::Display;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum FriendOrFoe {
    Friendly,
    Hostile,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum AbilityTarget {
    Single(u32),
    Group(FriendOrFoe),
    All,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CombatantAbility {
    pub ability_name: CombatantAbilityNames,
    pub level: u8,
}

impl CombatantAbility {
    pub fn new(ability_name: CombatantAbilityNames) -> Self {
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
        }
    }
}

#[derive(Debug, Clone, Hash, Eq, PartialEq, Serialize, Deserialize, PartialOrd, Ord)]
pub enum CombatantAbilityNames {
    Attack,
    ArmorBreak,
    HeatLance,
    Fire,
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
        }
    }
}

impl CombatantAbility {
    /// Create an instance of an ability with the default values
    pub fn create_by_name(name: &CombatantAbilityNames) -> CombatantAbility {
        match name {
            CombatantAbilityNames::Attack => {
                let mut ability = CombatantAbility::new(CombatantAbilityNames::Attack);
                ability.level = 1;
                ability
            }
            name => CombatantAbility::new(name.clone()),
        }
    }
}
