use self::inventory::CharacterInventory;
use self::outfit_new_warrior::outfit_new_warrior;
use crate::app_consts::error_messages;
use crate::combatants::abilities::CombatantAbility;
use crate::combatants::abilities::CombatantAbilityNames;
use crate::combatants::CombatantClass;
use crate::combatants::CombatantProperties;
use crate::errors::AppError;
use crate::game::RoguelikeRacerGame;
use crate::items::Item;
use crate::primatives::EntityProperties;
use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;
pub mod inventory;
pub mod outfit_new_warrior;

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
        game: &mut RoguelikeRacerGame,
        name: &str,
        combatant_class: CombatantClass,
        name_of_controlling_user: String,
    ) -> Character {
        let mut character = Character {
            name_of_controlling_user,
            entity_properties: EntityProperties {
                id: game.id_generator.get_next_entity_id(),
                name: name.to_owned(),
            },
            combatant_properties: CombatantProperties::new(&combatant_class, HashMap::new()),
            inventory: CharacterInventory::new(),
            unspent_ability_points: 1,
            actions_taken: 0,
        };

        character.combatant_properties.abilities.insert(
            CombatantAbilityNames::Attack,
            CombatantAbility::new(&CombatantAbilityNames::Attack),
        );

        match combatant_class {
            CombatantClass::Mage => {}
            CombatantClass::Rogue => {}
            CombatantClass::Warrior => outfit_new_warrior(game, &mut character),
            CombatantClass::Monster => {}
        }

        character
    }

    pub fn equip_item(&self, item_id: u32) -> Result<(), AppError> {
        let mut item = None;
        for item_in_inventory in &self.inventory.items {
            if item_in_inventory.entity_properties.id == item_id {
                item = Some(item_in_inventory);
            }
        }
        if item == None {
            return Err(AppError {
                error_type: crate::errors::AppErrorTypes::InvalidInput,
                message: error_messages::INVALID_ITEM_ID.to_string(),
            });
        }
        Ok(())
    }

    pub fn character_can_use_item(&self, item: Item) -> bool {
        let total_character_attributes = self.combatant_properties.get_total_attributes();
        if let Some(requirements) = &item.requirements {
            for (attribute, value) in requirements {
                let character_attribute_option = total_character_attributes.get(attribute);
                match character_attribute_option {
                    Some(attr_value) => {
                        if *attr_value >= *value as u16 {
                            continue;
                        } else {
                            return false;
                        }
                    }
                    None => return false,
                };
            }
        } else {
            return true;
        }
        true
    }
}
