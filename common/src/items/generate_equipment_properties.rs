use crate::primatives::MaxAndCurrent;

use super::{EquipmentProperties, EquipmentTypes, Item, ItemProperties, StatTypes};
use rand::prelude::*;
use strum::IntoEnumIterator;

impl Item {
    pub fn generate_equipment_properties(level: u16) -> ItemProperties {
        let mut rng = rand::thread_rng();
        let equipment_types: Vec<_> = EquipmentTypes::iter().collect();
        let equipment_type = *equipment_types.choose(&mut rand::thread_rng()).unwrap();
        let stat_types: Vec<_> = StatTypes::iter().collect();
        let bonus_stat_type = *stat_types.choose(&mut rand::thread_rng()).unwrap();

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

        // DETERMINE BONUS STATS
        let mut dexterity = 0;
        let mut strength = 0;
        let mut intelligence = 0;
        let mut bonus_stat_amount = 0;

        match equipment_type {
            EquipmentTypes::Ring | EquipmentTypes::Amulet | EquipmentTypes::TwoHandedWeapon => {
                bonus_stat_amount = rng.gen_range(level..level * 2)
            }
            _ => bonus_stat_amount = rng.gen_range(1..level),
        };

        match bonus_stat_type {
            StatTypes::Dexterity => dexterity = bonus_stat_amount,
            StatTypes::Strength => strength = bonus_stat_amount,
            StatTypes::Intelligence => intelligence = bonus_stat_amount,
        };

        let equipment_properties = EquipmentProperties {
            equipment_type,
            durability: Some(MaxAndCurrent {
                max: durability,
                current: durability,
            }),
            armor_class,
            damage,
            dexterity,
            intelligence,
            strength,
        };

        ItemProperties::Equipment(equipment_properties)
    }
}
