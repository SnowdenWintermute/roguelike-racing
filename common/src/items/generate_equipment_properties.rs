use std::collections::HashMap;

use super::{equipment::EquipmentTypes, EquipmentProperties, Item, ItemProperties};
use crate::{combatants::CombatAttributes, primatives::MaxAndCurrent};
use rand::prelude::*;
use strum::IntoEnumIterator;

impl Item {
    pub fn generate_equipment_properties(level: u16) -> ItemProperties {
        let mut rng = rand::thread_rng();
        let equipment_types: Vec<_> = EquipmentTypes::iter().collect();
        let equipment_type = *equipment_types.choose(&mut rand::thread_rng()).unwrap();
        let attribute_types: Vec<_> = CombatAttributes::iter().collect();
        let bonus_attribute_type = *attribute_types.choose(&mut rand::thread_rng()).unwrap();

        // DETERMINE BASE STATS
        let durability = rng.gen_range(1..=level) * 5;
        let mut armor_class = 0;
        let mut damage = 0;
        match equipment_type {
            EquipmentTypes::Helmet | EquipmentTypes::Shield => {
                armor_class = rng.gen_range(1..=level)
            }
            EquipmentTypes::BodyArmor => armor_class = rng.gen_range(level..=level * 2),
            EquipmentTypes::OneHandedWeapon => damage = rng.gen_range(1..=level),
            EquipmentTypes::TwoHandedWeapon => damage = rng.gen_range(level..=level * 2),
            _ => (),
        };

        let mut attributes = HashMap::new();
        let mut bonus_stat_amount;

        match equipment_type {
            EquipmentTypes::Ring | EquipmentTypes::Amulet | EquipmentTypes::TwoHandedWeapon => {
                bonus_stat_amount = rng.gen_range(level..level * 2)
            }
            _ => bonus_stat_amount = rng.gen_range(1..level),
        };

        attributes.insert(bonus_attribute_type, bonus_stat_amount);

        let equipment_properties = EquipmentProperties {
            equipment_type,
            durability: Some(MaxAndCurrent {
                max: durability,
                current: durability,
            }),
            attributes,
        };

        ItemProperties::Equipment(equipment_properties)
    }
}
