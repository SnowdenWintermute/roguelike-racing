#![allow(dead_code)]
use std::collections::HashMap;

use strum_macros::EnumIter;

use crate::character::combatant_properties::CombatantProperties;
use crate::character::items::CombatantEquipment;
use crate::game::id_generator::IdGenerator;
use crate::primatives::{EntityProperties, MaxAndCurrent};

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
    pub combatant_properties: CombatantProperties,
}

impl Monster {
    pub fn generate(id_generator: &mut IdGenerator, _level: u8) -> Monster {
        Monster {
            entity_properties: EntityProperties {
                id: id_generator.get_next_entity_id(),
                name: "some monster name".to_string(),
            },
            combatant_properties: CombatantProperties {
                combatant_class: crate::character::combatant_properties::CombatantClass::Monster,
                abilities: HashMap::new(),
                status_effects: Vec::new(),
                hit_points: MaxAndCurrent::new(10, 10),
                mana: MaxAndCurrent::new(10, 10),
                equipment: CombatantEquipment::new(),
            },
        }
    }
}
