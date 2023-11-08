#![allow(dead_code)]
use crate::combatants::CombatantClass;
use crate::combatants::CombatantProperties;
use crate::game::id_generator::IdGenerator;
use crate::primatives::EntityProperties;
use crate::primatives::MaxAndCurrent;
use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;
use strum_macros::EnumIter;

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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
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
            combatant_properties: CombatantProperties::new(CombatantClass::Monster, HashMap::new()),
        }
    }
}
