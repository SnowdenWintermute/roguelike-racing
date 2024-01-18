pub mod consumables;
pub mod equipment;
pub mod items_by_level;
use self::consumables::ConsumableProperties;
use self::equipment::equipment_generation::generate_equipment_properties_from_base_item;
use self::equipment::equipment_generation::name_equipment::name_equipment;
use self::equipment::equipment_generation::EquipmentPropertiesAndRequirements;
use self::equipment::EquipmentProperties;
use crate::app_consts::error_messages;
use crate::combatants::combat_attributes::CombatAttributes;
use crate::errors::AppError;
use crate::game::id_generator::IdGenerator;
use crate::primatives::EntityProperties;
use serde::Deserialize;
use serde::Serialize;
mod generate_consumable_properties;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ItemProperties {
    Consumable(ConsumableProperties),
    Equipment(EquipmentProperties),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Item {
    pub entity_properties: EntityProperties,
    pub item_level: u8,
    pub requirements: Option<HashMap<CombatAttributes, u8>>,
    pub item_properties: ItemProperties,
}

// const CHANCE_OF_CONSUMABLE_DROP: u16 = 20;

impl Item {
    pub fn generate(id_generator: &mut IdGenerator, level: u8) -> Item {
        let EquipmentPropertiesAndRequirements {
            equipment_properties,
            requirements,
        } = generate_equipment_properties_from_base_item(level);
        let item_name = name_equipment(&equipment_properties);
        println!("generated item: {item_name}");

        Item {
            entity_properties: EntityProperties {
                id: id_generator.get_next_entity_id(),
                name: item_name,
            },
            item_level: level as u8,
            requirements,
            item_properties: ItemProperties::Equipment(equipment_properties),
        }
    }

    pub fn get_equipment_properties(&self) -> Result<&EquipmentProperties, AppError> {
        match &self.item_properties {
            crate::items::ItemProperties::Consumable(_) => {
                return Err(AppError {
                    error_type: crate::errors::AppErrorTypes::InvalidInput,
                    message: error_messages::CANT_EQUIP_NON_EQUIPMENT.to_owned(),
                })
            }
            crate::items::ItemProperties::Equipment(equipment_properties) => {
                Ok(equipment_properties)
            }
        }
    }

    pub fn requirements_satisfied_by_attributes(
        &self,
        attributes: &HashMap<CombatAttributes, u16>,
    ) -> bool {
        if let Some(requirements) = &self.requirements {
            for (required_attribute, required_value) in requirements {
                if let Some(character_attribute) = attributes.get(&required_attribute) {
                    if *character_attribute < *required_value as u16 {
                        return false;
                    }
                } else {
                    return false;
                }
            }
        }
        true
    }
}
