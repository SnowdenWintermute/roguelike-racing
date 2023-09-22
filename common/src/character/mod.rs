#![allow(dead_code)]
use std::collections::HashMap;

use crate::game::id_generator::IdGenerator;
use crate::primatives::{EntityProperties, MaxAndCurrent};

use self::abilities::{CombatantAbilities, CombatantAbility};
use self::combatant_properties::{CombatantClass, CombatantProperties};
use self::items::{CharacterInventory, CombatantEquipment};

pub mod abilities;
pub mod combatant_properties;
pub mod items;

#[derive(Debug)]
pub struct Character {
    pub entity_properties: EntityProperties,
    pub combatant_properties: CombatantProperties,
    pub inventory: CharacterInventory,
    pub unspent_ability_points: u8,
    pub actions_taken: u8,
}

impl Character {
    pub fn new(
        id_generator: &mut IdGenerator,
        name: &str,
        combatant_class: CombatantClass,
    ) -> Character {
        let mut abilities = HashMap::<CombatantAbilities, CombatantAbility>::new();
        abilities.insert(
            CombatantAbilities::Attack,
            CombatantAbilities::new(&CombatantAbilities::Attack),
        );
        match combatant_class {
            CombatantClass::Mage => {
                abilities.insert(
                    CombatantAbilities::HeatLance,
                    CombatantAbilities::new(&CombatantAbilities::HeatLance),
                );
            }
            CombatantClass::Rogue => {
                abilities.insert(
                    CombatantAbilities::ShootArrow,
                    CombatantAbilities::new(&CombatantAbilities::ShootArrow),
                );
            }
            CombatantClass::Warrior => {
                abilities.insert(
                    CombatantAbilities::ArmorBreak,
                    CombatantAbilities::new(&CombatantAbilities::ArmorBreak),
                );
            }
            CombatantClass::Monster => {}
        }

        Character {
            entity_properties: EntityProperties {
                id: id_generator.get_next_entity_id(),
                name: name.to_owned(),
            },
            combatant_properties: CombatantProperties {
                combatant_class,
                hit_points: MaxAndCurrent::new(10, 10),
                mana: MaxAndCurrent::new(10, 10),
                status_effects: vec![],
                equipment: CombatantEquipment::new(),
                abilities,
            },
            inventory: CharacterInventory::new(),
            unspent_ability_points: 1,
            actions_taken: 0,
        }
    }
}
