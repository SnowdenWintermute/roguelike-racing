use super::abilities::CombatantAbilities;
use super::items::CombatantEquipment;
use crate::character::CombatantAbility;
use crate::primatives::MaxAndCurrent;
use crate::status_effects::StatusEffects;
use core::fmt;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum CombatantClass {
    Warrior,
    Mage,
    Rogue,
    Monster,
}

impl fmt::Display for CombatantClass {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CombatantClass::Warrior => write!(f, "Warrior"),
            CombatantClass::Mage => write!(f, "Mage"),
            CombatantClass::Rogue => write!(f, "Rogue"),
            CombatantClass::Monster => write!(f, "Monster"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CombatantProperties {
    pub combatant_class: CombatantClass,
    pub hit_points: MaxAndCurrent<u16>,
    pub mana: MaxAndCurrent<u16>,
    pub status_effects: Vec<StatusEffects>,
    pub equipment: CombatantEquipment,
    pub abilities: HashMap<CombatantAbilities, CombatantAbility>,
    // pub traits: HashSet<CombatantTraits>
    pub target_ids: Option<Vec<u32>>,
    pub selected_ability_slot: Option<u8>,
    pub selected_item_slot: Option<u8>,
}
