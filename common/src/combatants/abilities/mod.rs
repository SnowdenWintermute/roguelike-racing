pub mod get_combatant_ability_attributes;
use serde::Deserialize;
use serde::Serialize;
use std::fmt::Display;

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
            ability_name: CombatantAbilityNames::AttackMeleeMainhand,
            level: 0,
        }
    }
}

#[derive(Debug, Clone, Hash, Eq, PartialEq, Serialize, Deserialize, PartialOrd, Ord)]
pub enum CombatantAbilityNames {
    Attack,
    AttackMeleeMainhand,
    AttackMeleeOffhand,
    AttackRangedMainhand,
    Fire,
    Ice,
    Healing,
}

impl Display for CombatantAbilityNames {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CombatantAbilityNames::Attack => write!(f, "Attack"),
            CombatantAbilityNames::AttackMeleeMainhand => write!(f, "Attack"),
            CombatantAbilityNames::AttackMeleeOffhand => write!(f, "Attack"),
            CombatantAbilityNames::AttackRangedMainhand => write!(f, "Ranged Attack"),
            CombatantAbilityNames::Healing => write!(f, "Healing"),
            CombatantAbilityNames::Fire => write!(f, "Fire"),
            CombatantAbilityNames::Ice => write!(f, "Ice"),
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
            CombatantAbilityNames::Healing => {
                let mut ability = CombatantAbility::new(CombatantAbilityNames::Healing);
                ability.level = 1;
                ability
            }
            CombatantAbilityNames::Fire => {
                let mut ability = CombatantAbility::new(CombatantAbilityNames::Fire);
                ability.level = 1;
                ability
            }
            CombatantAbilityNames::Ice => {
                let mut ability = CombatantAbility::new(CombatantAbilityNames::Ice);
                ability.level = 1;
                ability
            }
            name => CombatantAbility::new(name.clone()),
        }
    }
}
