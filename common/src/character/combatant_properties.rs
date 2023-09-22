use super::abilities::CombatantAbilities;
use super::items::CombatantEquipment;
use crate::character::CombatantAbility;
use crate::items::Item;
use crate::primatives::MaxAndCurrent;
use crate::status_effects::StatusEffects;
use std::collections::HashMap;

#[derive(Debug)]
pub enum CombatantClass {
    Warrior,
    Mage,
    Rogue,
    Monster,
}

#[derive(Debug)]
pub struct CombatantProperties {
    pub combatant_class: CombatantClass,
    pub hit_points: MaxAndCurrent<u16>,
    pub mana: MaxAndCurrent<u16>,
    pub status_effects: Vec<StatusEffects>,
    pub equipment: CombatantEquipment,
    pub abilities: HashMap<CombatantAbilities, CombatantAbility>,
    // pub traits: HashSet<CombatantTraits>
    pub target_ids: Option<Vec<u32>>,
    pub selected_ability: Option<CombatantAbility>,
    pub selected_item: Option<Item>,
}
