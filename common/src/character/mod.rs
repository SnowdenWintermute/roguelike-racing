#![allow(dead_code)]
use self::inventory::CharacterInventory;
use crate::combatants::abilities::{CombatantAbilities, CombatantAbility};
use crate::combatants::{CombatAttributes, CombatantClass, CombatantProperties};
use crate::game::id_generator::IdGenerator;
use crate::items::equipment::EquipmentProperties;
use crate::items::{self, Item, ItemCategories};
use crate::primatives::{EntityProperties, MaxAndCurrent};
use serde::{Deserialize, Serialize};
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

        let mut starting_weapon_properties = EquipmentProperties {
            equipment_type: crate::items::equipment::EquipmentTypes::OneHandedWeapon,
            durability: Some(MaxAndCurrent {
                max: 10,
                current: 10,
            }),
            attributes: HashMap::new(),
        };

        starting_weapon_properties
            .attributes
            .insert(CombatAttributes::Damage, 1);

        let starting_weapon = Item {
            entity_properties: EntityProperties {
                id: 420,
                name: "starting weapon".to_string(),
            },
            item_level: 1,
            item_category: ItemCategories::Equipment,
            item_properties: items::ItemProperties::Equipment(starting_weapon_properties),
        };

        character.combatant_properties.equipment.insert(
            crate::items::equipment::EquipmentSlots::RightHand,
            starting_weapon,
        );

        character
    }
}
