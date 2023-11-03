use once_cell::sync::Lazy;
use rand::prelude::*;
use std::{collections::HashMap, default};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use super::{
    armors::{Armors, ARMOR_BY_LEVEL},
    Item,
};

#[derive(EnumIter)]
pub enum EquipmentCategories {
    Armor,
    Jewelry,
    MeleeWeapon,
    RangedWeapon,
    Shield,
}

#[derive(Debug)]
pub enum BaseItem {
    Armor(Armors),
    Jewelry,
    MeleeWeapon,
    RangedWeapon,
    Shield,
}

impl Item {
    pub fn generate_base_item(level: u16) -> BaseItem {
        let mut rng = rand::thread_rng();
        let categories: Vec<EquipmentCategories> = EquipmentCategories::iter().collect();
        let category = categories.choose(&mut rand::thread_rng()).unwrap();
        match category {
            // EquipmentCategories::Armor => {
            // }
            _ =>{
                let possible_base_armors_option = ARMOR_BY_LEVEL
                    .get(&level);
                if possible_base_armors_option.is_some() {
                    let possible_base_armors = possible_base_armors_option.unwrap();
                println!("base armors: {:#?}", possible_base_armors);
                return BaseItem::Armor(*possible_base_armors
                    .choose(&mut rand::thread_rng())
                    // .clone()
                    .unwrap())
                };

                BaseItem::Armor(Armors::Rags)
            }
            // EquipmentCategories::Jewelry => {

            // },
            // EquipmentCategories::MeleeWeapon => todo!(),
            // EquipmentCategories::RangedWeapon => todo!(),
            // EquipmentCategories::Shield => todo!(),
        }
    }
}

pub enum Weapons {
    Club,
    Mace,
    Morningstar,
    WarHammer,
    Stick,
    Staff,
    Maul,
    ShortSword,
    Sabre,
    Blade,
    BroadSword,
    BastardSword,
    TwoHandedSword,
    Katana,
    GreatAxe,
    Spear,
    Pike,
    ShortBow,
    HuntersBow,
    LongBow,
    CompositeBow,
    WarBow,
}
