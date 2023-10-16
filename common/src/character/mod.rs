#![allow(dead_code)]
use serde::{Deserialize, Serialize};

use crate::game::id_generator::IdGenerator;
use crate::primatives::{EntityProperties, MaxAndCurrent};
use std::collections::HashMap;

use self::abilities::{CombatantAbilities, CombatantAbility};
use self::combatant_properties::CombatantProperties;
use self::items::{CharacterInventory, CombatantEquipment};

pub mod abilities;
pub mod combatant_properties;
pub mod items;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Character {
    pub entity_properties: EntityProperties,
    pub combatant_properties: CombatantProperties,
    pub inventory: CharacterInventory,
    pub unspent_ability_points: u8,
    pub actions_taken: u8,
}

impl Character {
    pub fn new(
        id: u32,
        name: &str,
        combatant_class: combatant_properties::CombatantClass,
    ) -> Character {
        let mut abilities = HashMap::<CombatantAbilities, CombatantAbility>::new();
        abilities.insert(
            CombatantAbilities::Attack,
            CombatantAbilities::new(&CombatantAbilities::Attack),
        );
        match combatant_class {
            combatant_properties::CombatantClass::Mage => {
                abilities.insert(
                    CombatantAbilities::HeatLance,
                    CombatantAbilities::new(&CombatantAbilities::HeatLance),
                );
            }
            combatant_properties::CombatantClass::Rogue => {
                abilities.insert(
                    CombatantAbilities::ShootArrow,
                    CombatantAbilities::new(&CombatantAbilities::ShootArrow),
                );
            }
            combatant_properties::CombatantClass::Warrior => {
                abilities.insert(
                    CombatantAbilities::ArmorBreak,
                    CombatantAbilities::new(&CombatantAbilities::ArmorBreak),
                );
            }
            combatant_properties::CombatantClass::Monster => {}
        }

        Character {
            entity_properties: EntityProperties {
                id,
                name: name.to_owned(),
            },
            combatant_properties: CombatantProperties {
                combatant_class,
                hit_points: MaxAndCurrent::new(10, 10),
                mana: MaxAndCurrent::new(10, 10),
                status_effects: vec![],
                equipment: CombatantEquipment::new(),
                abilities,
                selected_item_slot: None,
                selected_ability_slot: None,
                target_ids: None,
            },
            inventory: CharacterInventory::new(),
            unspent_ability_points: 1,
            actions_taken: 0,
        }
    }
}
