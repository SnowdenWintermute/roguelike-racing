mod get_consumable_properties;
use rand::seq::SliceRandom;
use serde::Deserialize;
use serde::Serialize;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Debug, EnumIter, Clone, Copy, PartialEq, Serialize, Deserialize, Eq)]
pub enum ConsumableTypes {
    Autoinjector,
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

impl ConsumableTypes {
    pub fn random(&self) -> Self {
        let vec: Vec<_> = ConsumableTypes::iter().collect();
        *vec.choose(&mut rand::thread_rng()).unwrap()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ConsumableProperties {
    pub consumable_type: ConsumableTypes,
    pub uses_remaining: u8,
}
