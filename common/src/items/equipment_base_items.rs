use super::{
    body_armor::{body_armors_by_level::BODY_ARMORS_BY_LEVEL, BodyArmors},
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
    Helm,
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
                let possible_base_armors_option = BODY_ARMORS_BY_LEVEL
                    .get(&level);
                if possible_base_armors_option.is_some() {
                    let possible_base_armors = possible_base_armors_option.unwrap();
                return BaseItem::Armor(*possible_base_armors
                    .choose(&mut rand::thread_rng())
                    // .clone()
                    .unwrap())
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
