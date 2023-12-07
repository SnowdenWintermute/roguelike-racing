pub mod get_combatant_ability_attributes;
mod get_default_target_ids;
mod targets_are_valid;
use self::get_combatant_ability_attributes::TargetingScheme;
use serde::Deserialize;
use serde::Serialize;
use std::fmt::Display;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CombatantAbility {
    pub ability_name: CombatantAbilityNames,
    pub level: u8,
    pub selected_targeting_scheme: TargetingScheme,
    pub most_recently_targeted: Option<Vec<u32>>,
}

impl CombatantAbility {
    pub fn new(ability_name: CombatantAbilityNames) -> Self {
        CombatantAbility {
            selected_targeting_scheme: ability_name.get_attributes().targeting_schemes[0].clone(),
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
            selected_targeting_scheme: TargetingScheme::Single,
            most_recently_targeted: None,
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
