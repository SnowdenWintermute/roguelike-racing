use super::{
    body_armor::{body_armor_generation_templates::BODY_ARMORS_BY_LEVEL, BodyArmors},
    headgear::{headgear_generation_templates::HEADGEARS_BY_LEVEL, HeadGears},
    Item,
};
use crate::items::equipment::EquipmentTypes;
use core::panic;
use once_cell::sync::Lazy;
use rand::prelude::*;
use std::{collections::HashMap, default};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Debug)]
pub enum BaseItem {
    Armor(BodyArmors),
    HeadGear(HeadGears),
    Jewelry,
    MeleeWeapon,
    RangedWeapon,
    Shield,
}

impl Item {
    pub fn generate_base_item(level: u8) -> BaseItem {
        let mut rng = rand::thread_rng();
        let categories: Vec<EquipmentTypes> = EquipmentTypes::iter().collect();
        let category = categories.choose(&mut rand::thread_rng()).unwrap();
        match category {
            // EquipmentCategories::Armor => {
            // }
            _ =>{
                let possible_base_armors_option = HEADGEARS_BY_LEVEL
                    .get(&level);
                if possible_base_armors_option.is_some() {
                    let possible_base_armors = possible_base_armors_option.unwrap();
                return BaseItem::HeadGear(HeadGears::Ribbon)
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
}
