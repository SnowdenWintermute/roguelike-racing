use crate::items::equipment::body_armors::BodyArmors;
use crate::items::equipment::head_gears::HeadGears;
use crate::items::equipment::one_handed_melee_weapons::OneHandedMeleeWeapons;
use crate::items::equipment::EquipmentTypes;
use core::panic;
use once_cell::sync::Lazy;
use rand::prelude::*;
use std::collections::HashMap;
use std::default;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use super::equipment_generation_templates::head_gear_generation_templates::HEAD_GEARS_BY_LEVEL;

#[derive(Debug)]
pub enum BaseEquipment {
    BodyArmor(BodyArmors),
    HeadGear(HeadGears),
    Jewelry,
    OneHandedMeleeWeapon(OneHandedMeleeWeapons),
    TwoHandedMeleeWeapon,
    TwoHandedRangedWeapon,
    Shield,
}

pub fn generate_base_equipment(level: u8) -> BaseEquipment {
    let mut rng = rand::thread_rng();
    let categories: Vec<EquipmentTypes> = EquipmentTypes::iter().collect();
    let category = categories.choose(&mut rand::thread_rng()).unwrap();
    match category {
            // EquipmentCategories::Armor => {
            // }
            _ =>{
                let possible_base_armors_option = HEAD_GEARS_BY_LEVEL
                    .get(&level);
                if possible_base_armors_option.is_some() {
                    let possible_base_armors = possible_base_armors_option.unwrap();
                // return BaseItem::HeadGear(HeadGears::Ribbon)
                // return BaseEquipment::OneHandedMeleeWeapon(OneHandedMeleeWeapons::BastardSword)
                return BaseEquipment::BodyArmor(BodyArmors::ShardPlate);
                // let possible_base_armors_option = BODY_ARMORS_BY_LEVEL
                //     .get(&level);
                // if possible_base_armors_option.is_some() {
                //     let possible_base_armors = possible_base_armors_option.unwrap();
                // return BaseItem::Armor(*possible_base_armors
                //     .choose(&mut rand::thread_rng())
                //     // .clone()
                //     .unwrap())
                } else {
                    panic!("tried to generate an armor but no possible base items were found")
                }
            }
            // EquipmentCategories::Jewelry => {

            // },
            // EquipmentCategories::MeleeWeapon => todo!(),
            // EquipmentCategories::RangedWeapon => todo!(),
            // EquipmentCategories::Shield => todo!(),
        }
}
