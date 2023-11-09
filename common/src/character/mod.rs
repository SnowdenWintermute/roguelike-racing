#![allow(dead_code)]
use self::inventory::CharacterInventory;
use crate::combatants::abilities::CombatantAbility;
use crate::combatants::abilities::CombatantAbilityNames;
use crate::combatants::CombatAttributes;
use crate::combatants::CombatantClass;
use crate::combatants::CombatantProperties;
use crate::primatives::EntityProperties;
use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;
pub mod inventory;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Character {
    pub name_of_controlling_user: String,
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
        combatant_class: CombatantClass,
        name_of_controlling_user: String,
    ) -> Character {
        let mut abilities = HashMap::<CombatantAbilityNames, CombatantAbility>::new();
        abilities.insert(
            CombatantAbilityNames::Attack,
            CombatantAbility::new(&CombatantAbilityNames::Attack),
        );
        match combatant_class {
            CombatantClass::Mage => {
                abilities.insert(
                    CombatantAbilityNames::HeatLance,
                    CombatantAbility::new(&CombatantAbilityNames::HeatLance),
                );
            }
            CombatantClass::Rogue => {
                abilities.insert(
                    CombatantAbilityNames::ShootArrow,
                    CombatantAbility::new(&CombatantAbilityNames::ShootArrow),
                );
            }
            CombatantClass::Warrior => {
                abilities.insert(
                    CombatantAbilityNames::ArmorBreak,
                    CombatantAbility::new(&CombatantAbilityNames::ArmorBreak),
                );
                abilities.insert(
                    CombatantAbilityNames::Heal,
                    CombatantAbility::new(&CombatantAbilityNames::Heal),
                );
            }
            CombatantClass::Monster => {}
        }

        let mut character = Character {
            name_of_controlling_user,
            entity_properties: EntityProperties {
                id,
                name: name.to_owned(),
            },
            combatant_properties: CombatantProperties::new(combatant_class, abilities),
            inventory: CharacterInventory::new(),
            unspent_ability_points: 1,
            actions_taken: 0,
        };

        character
            .combatant_properties
            .inherent_attributes
            .insert(CombatAttributes::Damage, 4);

        character
    }
}
