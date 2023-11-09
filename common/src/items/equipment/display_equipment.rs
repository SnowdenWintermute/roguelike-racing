use crate::items::{equipment::EquipmentTypes, Item};

use super::{
    armor_properties::ArmorProperties, weapon_properties::WeaponProperties, EquipmentProperties,
};
use core::fmt;
use std::fmt::Display;

fn print_weapon<T>(
    base_item: T,
    weapon_properties: &WeaponProperties,
    equipment_properties: &EquipmentProperties,
    item_level: u8,
) -> String
where
    T: Display,
{
    let mut output = String::new();
    output.push_str(format!("{} ilvl {item_level}\n", base_item).as_str());
    output.push_str(
        format!(
            "Damage: {}-{} ",
            weapon_properties.damage.min, weapon_properties.damage.max
        )
        .as_str(),
    );
    let mut classifications = String::new();
    classifications.push_str("[");
    for (i, classification) in weapon_properties.damage_classifications.iter().enumerate() {
        classifications.push_str(format!("{}", classification).as_str());
        if i != weapon_properties.damage_classifications.len() - 1 {
            classifications.push_str(", ")
        } else {
            classifications.push_str("]\n")
        }
    }
    output.push_str(format!("{}", classifications).as_str());
    output.push_str(print_equipment_properties(equipment_properties).as_str());
    output
}

fn print_armor<T>(
    base_item: T,
    armor_properties: &ArmorProperties,
    equipment_properties: &EquipmentProperties,
    item_level: u8,
) -> String
where
    T: Display,
{
    let mut output = String::new();
    output.push_str(format!("{} ilvl {item_level}\n", base_item).as_str());
    output.push_str(format!("Armor Category: {}\n", armor_properties.armor_category).as_str());
    output.push_str(format!("Armor Class: {}\n", armor_properties.armor_class).as_str());
    output.push_str(print_equipment_properties(equipment_properties).as_str());
    output
}

fn print_equipment_properties(properties: &EquipmentProperties) -> String {
    let mut output = String::new();
    if let Some(durability) = &properties.durability {
        output
            .push_str(format!("Durability: {}/{}\n", durability.current, durability.max).as_str());
    }
    for affix in &properties.affixes {
        output.push_str(format!("{}\n", affix).as_str());
    }
    for attribute in &properties.attributes {
        output.push_str(format!("{}: {}\n", attribute.0, attribute.1).as_str());
    }
    if let Some(equipment_traits) = &properties.traits {
        for equipment_trait in equipment_traits {
            output.push_str(format!("{} \n", equipment_trait).as_str());
        }
    }

    output
}

impl fmt::Display for Item {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut output = String::new();
        match &self.item_properties {
            crate::items::ItemProperties::Consumable(_) => todo!(),
            crate::items::ItemProperties::Equipment(equipment_properties) => {
                match &equipment_properties.equipment_type {
                    EquipmentTypes::BodyArmor(base_item, properties) => output.push_str(
                        print_armor(
                            base_item,
                            &properties,
                            &equipment_properties,
                            self.item_level,
                        )
                        .as_str(),
                    ),
                    EquipmentTypes::HeadGear(base_item, properties) => output.push_str(
                        print_armor(
                            base_item,
                            &properties,
                            &equipment_properties,
                            self.item_level,
                        )
                        .as_str(),
                    ),
                    EquipmentTypes::Ring => {
                        output.push_str(format!("Ring ilvl {}\n", self.item_level).as_str());
                        output.push_str(print_equipment_properties(&equipment_properties).as_str());
                    }
                    EquipmentTypes::Amulet => {
                        output.push_str(format!("Amulet ilvl {}\n", self.item_level).as_str());
                        output.push_str(print_equipment_properties(&equipment_properties).as_str());
                    }
                    EquipmentTypes::OneHandedMeleeWeapon(base_item, properties) => {
                        output.push_str(
                            print_weapon(
                                base_item,
                                &properties,
                                &equipment_properties,
                                self.item_level,
                            )
                            .as_str(),
                        );
                    }
                    EquipmentTypes::TwoHandedMeleeWeapon(base_item, properties) => {
                        output.push_str(
                            print_weapon(
                                base_item,
                                &properties,
                                &equipment_properties,
                                self.item_level,
                            )
                            .as_str(),
                        );
                    }
                    EquipmentTypes::TwoHandedRangedWeapon(base_item, properties) => {
                        output.push_str(
                            print_weapon(
                                base_item,
                                &properties,
                                &equipment_properties,
                                self.item_level,
                            )
                            .as_str(),
                        );
                    }
                    EquipmentTypes::Shield(base_item, properties) => {
                        let mut output = String::new();
                        output
                            .push_str(format!("{} ilvl {}\n", base_item, self.item_level).as_str());
                        output.push_str(format!("Shield Size: {}\n", properties.size).as_str());
                        output.push_str(
                            format!("Armor Class: {}\n", properties.armor_class).as_str(),
                        );
                        output.push_str(print_equipment_properties(&equipment_properties).as_str());
                    }
                }
            }
        }
        write!(f, "{}", output)
    }
}
