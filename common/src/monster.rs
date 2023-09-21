#![allow(dead_code)]
use strum_macros::EnumIter;

use crate::{
    character::items::CharacterEquipment,
    game::IdGenerator,
    primatives::{EntityProperties, MaxAndCurrent},
};

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
    pub entity_properties: EntityProperties,
    pub hit_points: MaxAndCurrent<u16>,
    pub mana: MaxAndCurrent<u16>,
    pub equipment: CharacterEquipment,
    pub special_trait: Option<MonsterTraits>,
    pub special_ability: Option<MonsterAbilities>,
}

impl Monster {
    pub fn generate(id_generator: &mut IdGenerator, _level: u8) -> Monster {
        Monster {
            entity_properties: EntityProperties {
                id: id_generator.get_next_entity_id(),
                name: "some monster name".to_string(),
            },
            hit_points: MaxAndCurrent::new(10, 10),
            mana: MaxAndCurrent::new(10, 10),
            equipment: CharacterEquipment::new(),
            special_trait: None,
            special_ability: None,
        }
    }
}
