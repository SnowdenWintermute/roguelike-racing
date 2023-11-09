use super::equipment_generation_templates::body_armor_generation_templates::BODY_ARMORS_BY_LEVEL;
use super::equipment_generation_templates::head_gear_generation_templates::HEAD_GEARS_BY_LEVEL;
use super::equipment_generation_templates::one_handed_melee_weapon_generation_templates::ONE_HANDED_MELEE_WEAPONS_BY_LEVEL;
use super::equipment_generation_templates::shield_generation_templates::SHIELDS_BY_LEVEL;
use super::equipment_generation_templates::two_handed_melee_weapon_generation_templates::TWO_HANDED_MELEE_WEAPONS_BY_LEVEL;
use super::equipment_generation_templates::two_handed_ranged_weapon_generation_templates::TWO_HANDED_RANGED_WEAPONS_BY_LEVEL;
use crate::items::equipment::body_armors::BodyArmors;
use crate::items::equipment::head_gears::HeadGears;
use crate::items::equipment::jewelries::Jewelries;
use crate::items::equipment::one_handed_melee_weapons::OneHandedMeleeWeapons;
use crate::items::equipment::shield_properties::ShieldProperties;
use crate::items::equipment::shields::Shields;
use crate::items::equipment::two_handed_melee_weapons::TwoHandedMeleeWeapons;
use crate::items::equipment::two_handed_ranged_weapons::TwoHandedRangedWeapons;
use crate::items::equipment::EquipmentTypes;
use core::panic;
use rand::prelude::*;
use strum::IntoEnumIterator;

#[derive(Debug)]
pub enum BaseEquipment {
    BodyArmor(BodyArmors),
    HeadGear(HeadGears),
    Jewelry(Jewelries),
    OneHandedMeleeWeapon(OneHandedMeleeWeapons),
    TwoHandedMeleeWeapon(TwoHandedMeleeWeapons),
    TwoHandedRangedWeapon(TwoHandedRangedWeapons),
    Shield(Shields),
}

fn choose_base_item<T>(options: Option<&Vec<T>>) -> T
where
    T: Clone,
{
    if let Some(base_items) = options {
        let base_item = base_items.choose(&mut rand::thread_rng()).unwrap();
        return base_item.clone();
    } else {
        panic!("tried to generate an item but no possible base items were found")
    }
}

pub fn generate_base_equipment(level: u8) -> BaseEquipment {
    let categories: Vec<EquipmentTypes> = EquipmentTypes::iter().collect();
    let category = categories.choose(&mut rand::thread_rng()).unwrap();
    // let category = EquipmentTypes::Shield(Shields::Buckler, ShieldProperties::default());
    match category {
        EquipmentTypes::HeadGear(..) => {
            let possible_base_items = HEAD_GEARS_BY_LEVEL.get(&level);
            let base_item = choose_base_item(possible_base_items);
            BaseEquipment::HeadGear(base_item)
        }
        EquipmentTypes::BodyArmor(_, _) => {
            let possible_base_items = BODY_ARMORS_BY_LEVEL.get(&level);
            let base_item = choose_base_item(possible_base_items);
            BaseEquipment::BodyArmor(base_item)
        }
        EquipmentTypes::Ring => BaseEquipment::Jewelry(Jewelries::Ring),
        EquipmentTypes::Amulet => BaseEquipment::Jewelry(Jewelries::Amulet),
        EquipmentTypes::OneHandedMeleeWeapon(_, _) => {
            let possible_base_items = ONE_HANDED_MELEE_WEAPONS_BY_LEVEL.get(&level);
            let base_item = choose_base_item(possible_base_items);
            BaseEquipment::OneHandedMeleeWeapon(base_item)
        }
        EquipmentTypes::TwoHandedMeleeWeapon(_, _) => {
            let possible_base_items = TWO_HANDED_MELEE_WEAPONS_BY_LEVEL.get(&level);
            let base_item = choose_base_item(possible_base_items);
            BaseEquipment::TwoHandedMeleeWeapon(base_item)
        }
        EquipmentTypes::TwoHandedRangedWeapon(_, _) => {
            let possible_base_items = TWO_HANDED_RANGED_WEAPONS_BY_LEVEL.get(&level);
            let base_item = choose_base_item(possible_base_items);
            BaseEquipment::TwoHandedRangedWeapon(base_item)
        }
        EquipmentTypes::Shield(_, _) => {
            let possible_base_items = SHIELDS_BY_LEVEL.get(&level);
            let base_item = choose_base_item(possible_base_items);
            BaseEquipment::Shield(base_item)
        }
    }
}
