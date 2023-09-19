#![allow(dead_code)]
use strum_macros::EnumIter;

use crate::{character::CharacterEquipment, primatives::MaxAndCurrent};

#[derive(Debug, EnumIter, Clone, Copy, PartialEq)]
pub enum MonsterTraits {
    ManaShield,
    HpRegen,
    AcidicSkin,
    AbrasiveArmor,
}

#[derive(Debug, EnumIter, Clone, Copy, PartialEq)]
pub enum MonsterAbilities {
    PoisonSting,
    AcidicStrike,
    HeatLance,
    Thorns,
}

#[derive(Debug)]
pub struct Monster {
    hit_points: MaxAndCurrent<u16>,
    mana: MaxAndCurrent<u16>,
    equipment: CharacterEquipment,
    special_trait: Option<MonsterTraits>,
    special_ability: Option<MonsterAbilities>,
}

impl Monster {
    pub fn generate(_level: u8) -> Monster {
        Monster {
            hit_points: MaxAndCurrent::new(10, 10),
            mana: MaxAndCurrent::new(10, 10),
            equipment: CharacterEquipment::new(),
            special_trait: None,
            special_ability: None,
        }
    }
}
