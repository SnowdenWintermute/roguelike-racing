pub mod attributes_per_level;
use core::fmt;
use once_cell::sync::Lazy;
use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;
use strum_macros::EnumIter;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, EnumIter)]
pub enum CombatantClass {
    Warrior,
    Mage,
    Rogue,
}

impl fmt::Display for CombatantClass {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CombatantClass::Warrior => write!(f, "Warrior"),
            CombatantClass::Mage => write!(f, "Mage"),
            CombatantClass::Rogue => write!(f, "Rogue"),
        }
    }
}

pub static COMBATANT_CLASS_DESCRIPTIONS: Lazy<HashMap<CombatantClass, &str>> = Lazy::new(|| {
    HashMap::from([
        (
            CombatantClass::Warrior,
            "A strong and tough fighter specializing in melee combat",
        ),
        (
            CombatantClass::Mage,
            "A spellcaster with a talent for elemental magic and healing",
        ),
        (
            CombatantClass::Rogue,
            "An accurate and swift expert in both melee and ranged attacks",
        ),
    ])
});
