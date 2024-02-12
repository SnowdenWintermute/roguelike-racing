use super::CombatAttributes;
use super::CombatantProperties;
use crate::app_consts::AGI_TO_EVASION_RATIO;
use crate::app_consts::AGI_TO_SPEED_RATIO;
use crate::app_consts::DEX_TO_ACCURACY_RATIO;
use crate::app_consts::INT_TO_FOCUS_RATIO;
use crate::app_consts::INT_TO_MP_RATIO;
use crate::app_consts::VIT_TO_HP_RATIO;
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
                    // @TODO - add the %armor class trait to item generation and calculate it here
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

        calculate_and_add_derived_attribute(
            &mut total_attributes,
            &CombatAttributes::Dexterity,
            CombatAttributes::Accuracy,
            DEX_TO_ACCURACY_RATIO,
        );

        calculate_and_add_derived_attribute(
            &mut total_attributes,
            &CombatAttributes::Intelligence,
            CombatAttributes::Focus,
            INT_TO_FOCUS_RATIO,
        );

        calculate_and_add_derived_attribute(
            &mut total_attributes,
            &CombatAttributes::Intelligence,
            CombatAttributes::Mp,
            INT_TO_MP_RATIO,
        );

        calculate_and_add_derived_attribute(
            &mut total_attributes,
            &CombatAttributes::Agility,
            CombatAttributes::Evasion,
            AGI_TO_EVASION_RATIO,
        );

        calculate_and_add_derived_attribute(
            &mut total_attributes,
            &CombatAttributes::Agility,
            CombatAttributes::Speed,
            AGI_TO_SPEED_RATIO,
        );

        calculate_and_add_derived_attribute(
            &mut total_attributes,
            &CombatAttributes::Vitality,
            CombatAttributes::Hp,
            VIT_TO_HP_RATIO,
        );

        total_attributes
    }
}

fn calculate_and_add_derived_attribute(
    total_attributes: &mut HashMap<CombatAttributes, u16>,
    main: &CombatAttributes,
    derived: CombatAttributes,
    ratio: u16,
) {
    let total_main_option = total_attributes.get(main);
    let total_secondary = total_attributes.get(&derived).unwrap_or_else(|| &0);
    if let Some(main) = total_main_option {
        let derived_secondary_bonus = ratio * main;
        let total = total_secondary + derived_secondary_bonus;
        total_attributes.insert(derived, total);
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
