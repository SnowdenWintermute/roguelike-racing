use super::CombatAttributes;
use super::CombatantProperties;
use crate::app_consts::AGI_TO_SPEED_RATIO;
use crate::app_consts::DEX_TO_ACCURACY_RATIO;
use std::collections::HashMap;
use strum::IntoEnumIterator;

impl CombatantProperties {
    pub fn get_total_attributes(&self) -> HashMap<CombatAttributes, u16> {
        let mut total_attributes = HashMap::new();
        for attribute in CombatAttributes::iter() {
            total_attributes.insert(attribute, 0);
        }

        add_attributes_to_accumulator(&self.inherent_attributes, &mut total_attributes);

        for (_slot, item) in &self.equipment {
            match &item.item_properties {
                crate::items::ItemProperties::Consumable(_) => (),
                crate::items::ItemProperties::Equipment(equipment) => {
                    add_attributes_to_accumulator(&equipment.attributes, &mut total_attributes);
                    let base_ac = equipment.get_base_armor_class();
                    total_attributes
                        .entry(CombatAttributes::ArmorClass)
                        .and_modify(|value| *value += base_ac as u16)
                        .or_insert(base_ac as u16);
                }
            }
        }
        // after adding up attributes, determine if any equipped item still doesn't meet attribute
        // requirements, if so, remove it's attributes from the total
        for (_slot, item) in &self.equipment {
            let equipped_item_is_usable =
                item.requirements_satisfied_by_attributes(&total_attributes);
            if !equipped_item_is_usable {
                match &item.item_properties {
                    crate::items::ItemProperties::Consumable(_) => (),
                    crate::items::ItemProperties::Equipment(equipment) => {
                        remove_attributes_from_accumulator(
                            &equipment.attributes,
                            &mut total_attributes,
                        );
                        let base_ac = equipment.get_base_armor_class();
                        total_attributes
                            .entry(CombatAttributes::ArmorClass)
                            .and_modify(|value| *value -= base_ac as u16);
                    }
                }
            }
        }

        // derive accuracy from +acc, inherant, and all Dex
        let total_dex_option = total_attributes.get(&CombatAttributes::Dexterity);
        let total_acc = total_attributes
            .get(&CombatAttributes::Accuracy)
            .unwrap_or_else(|| &0);
        if let Some(dex) = total_dex_option {
            let accuracy_from_dex = DEX_TO_ACCURACY_RATIO * dex;
            total_attributes.insert(CombatAttributes::Accuracy, total_acc + accuracy_from_dex);
        }

        // derrive speed from agility and +speed
        let total_agility_option = total_attributes.get(&CombatAttributes::Agility);
        let total_speed = total_attributes
            .get(&CombatAttributes::Speed)
            .unwrap_or_else(|| &0);
        if let Some(agility) = total_agility_option {
            let speed_from_agility = AGI_TO_SPEED_RATIO * agility;
            total_attributes.insert(CombatAttributes::Speed, total_speed + speed_from_agility);
        }

        total_attributes
    }
}

pub fn add_attributes_to_accumulator(
    attr: &HashMap<CombatAttributes, u16>,
    acc: &mut HashMap<CombatAttributes, u16>,
) {
    for (attribute, number) in attr {
        if let Some(value) = acc.get_mut(attribute) {
            *value += number
        }
    }
}

pub fn remove_attributes_from_accumulator(
    attr: &HashMap<CombatAttributes, u16>,
    acc: &mut HashMap<CombatAttributes, u16>,
) {
    for (attribute, number) in attr {
        if let Some(value) = acc.get_mut(attribute) {
            *value -= number
        }
    }
}
