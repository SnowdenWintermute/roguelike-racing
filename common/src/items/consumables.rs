use crate::combatants::abilities::get_combatant_ability_attributes::TargetCategories;
use crate::combatants::abilities::get_combatant_ability_attributes::TargetingScheme;
use serde::Deserialize;
use serde::Serialize;
use strum_macros::EnumIter;

#[derive(Debug, EnumIter, Clone, Copy, PartialEq, Serialize, Deserialize, Eq)]
pub enum ConsumableTypes {
    RoomFinder,
    RepairKit,
    UpgradeKit,
    SmokeBomb,
    MilkDrink,
    FruitDrink,
    MonsterScanner,
    Antidote,
    Grenade,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ConsumableProperties {
    pub consumable_type: ConsumableTypes,
    pub uses_remaining: u8,
    pub combat_use_only: bool,
    pub requires_combat_turn: bool,
    pub targeting_schemes: Vec<TargetingScheme>,
    pub valid_targets: TargetCategories,
}
