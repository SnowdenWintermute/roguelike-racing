mod get_consumable_properties;
mod get_consumable_requirements;
use std::fmt::Display;

use rand::seq::SliceRandom;
use serde::Deserialize;
use serde::Serialize;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Debug, EnumIter, Clone, Copy, PartialEq, Serialize, Deserialize, Eq)]
pub enum ConsumableTypes {
    HpAutoinjector,
    Grenade,
    SmokeBomb,
    // RoomFinder,
    // RepairKit,
    // UpgradeKit,
    // MilkDrink,
    // FruitDrink,
    // MonsterScanner,
    // Antidote,
}

impl Display for ConsumableTypes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let to_write = match self {
            ConsumableTypes::HpAutoinjector => "HP Autoinjector",
            ConsumableTypes::Grenade => "Grenade",
            ConsumableTypes::SmokeBomb => "Smoke Bomb",
        };
        write!(f, "{}", to_write)
    }
}

impl ConsumableTypes {
    pub fn random() -> Self {
        let vec: Vec<_> = ConsumableTypes::iter().collect();
        *vec.choose(&mut rand::thread_rng()).unwrap()
    }

    pub fn get_description(&self) -> &str {
        match self {
            ConsumableTypes::HpAutoinjector => "Heal a friendly target.",
            ConsumableTypes::Grenade => "Damage a group of hostile targets.",
            ConsumableTypes::SmokeBomb => {
                "Apply a temporary Evasion bonus to a group of friendly targets."
            }
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ConsumableProperties {
    pub consumable_type: ConsumableTypes,
    pub uses_remaining: u8,
}
