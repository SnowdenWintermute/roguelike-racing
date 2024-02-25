use super::equipment_generation_templates::body_armor_generation_templates::BODY_ARMORS_BY_LEVEL;
use super::equipment_generation_templates::head_gear_generation_templates::HEAD_GEARS_BY_LEVEL;
use super::equipment_generation_templates::one_handed_melee_weapon_generation_templates::ONE_HANDED_MELEE_WEAPONS_BY_LEVEL;
use super::equipment_generation_templates::shield_generation_templates::SHIELDS_BY_LEVEL;
use super::equipment_generation_templates::two_handed_melee_weapon_generation_templates::TWO_HANDED_MELEE_WEAPONS_BY_LEVEL;
use super::equipment_generation_templates::two_handed_ranged_weapon_generation_templates::TWO_HANDED_RANGED_WEAPONS_BY_LEVEL;
use crate::app_consts::error_messages;
use crate::errors::AppError;
use crate::items::equipment::body_armors::BodyArmors;
use crate::items::equipment::head_gears::HeadGears;
use crate::items::equipment::jewelries::Jewelries;
use crate::items::equipment::one_handed_melee_weapons::OneHandedMeleeWeapons;
use crate::items::equipment::shields::Shields;
use crate::items::equipment::two_handed_melee_weapons::TwoHandedMeleeWeapons;
use crate::items::equipment::two_handed_ranged_weapons::TwoHandedRangedWeapons;
use crate::items::equipment::EquipmentTypes;
use crate::utils::server_log;
use core::fmt::Debug;
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

fn choose_base_item<T>(options: Option<&Vec<T>>) -> Result<T, AppError>
where
    T: Clone + Debug,
{
    if let Some(base_items) = options {
        let base_item_option = base_items.choose(&mut rand::thread_rng());
        if let Some(base_item) = base_item_option {
            return Ok(base_item.clone());
        } else {
            return Err(AppError {
                error_type: crate::errors::AppErrorTypes::ServerError,
                message: error_messages::NO_BASE_EQUIPMENT_FOUND.to_string(),
            });
        }
    } else {
        return Err(AppError {
            error_type: crate::errors::AppErrorTypes::ServerError,
            message: error_messages::NO_BASE_EQUIPMENT_FOUND.to_string(),
        });
    }
}

pub fn generate_base_equipment(level: u8) -> BaseEquipment {
    let categories: Vec<EquipmentTypes> = EquipmentTypes::iter().collect();
    let category = categories.choose(&mut rand::thread_rng()).unwrap();
    server_log(&format!(
        "generating random equipment in category: {category}"
    ));
    match category {
        EquipmentTypes::BodyArmor(_, _) => {
            let possible_base_items = BODY_ARMORS_BY_LEVEL.get(&level);
            let base_item_result = choose_base_item(possible_base_items);
            if let Ok(base_item) = base_item_result {
                BaseEquipment::BodyArmor(base_item)
            } else {
                generate_base_equipment(level)
            }
        }
        EquipmentTypes::HeadGear(..) => {
            let possible_base_items = HEAD_GEARS_BY_LEVEL.get(&level);
            let base_item_result = choose_base_item(possible_base_items);
            if let Ok(base_item) = base_item_result {
                BaseEquipment::HeadGear(base_item)
            } else {
                generate_base_equipment(level)
            }
        }
        EquipmentTypes::Ring => BaseEquipment::Jewelry(Jewelries::Ring),
        EquipmentTypes::Amulet => BaseEquipment::Jewelry(Jewelries::Amulet),
        EquipmentTypes::OneHandedMeleeWeapon(_, _) => {
            let possible_base_items = ONE_HANDED_MELEE_WEAPONS_BY_LEVEL.get(&level);
            let base_item_result = choose_base_item(possible_base_items);
            if let Ok(base_item) = base_item_result {
                BaseEquipment::OneHandedMeleeWeapon(base_item)
            } else {
                generate_base_equipment(level)
            }
        }
        EquipmentTypes::TwoHandedMeleeWeapon(_, _) => {
            let possible_base_items = TWO_HANDED_MELEE_WEAPONS_BY_LEVEL.get(&level);
            let base_item_result = choose_base_item(possible_base_items);
            if let Ok(base_item) = base_item_result {
                BaseEquipment::TwoHandedMeleeWeapon(base_item)
            } else {
                generate_base_equipment(level)
            }
        }
        EquipmentTypes::TwoHandedRangedWeapon(_, _) => {
            let possible_base_items = TWO_HANDED_RANGED_WEAPONS_BY_LEVEL.get(&level);
            let base_item_result = choose_base_item(possible_base_items);
            if let Ok(base_item) = base_item_result {
                BaseEquipment::TwoHandedRangedWeapon(base_item)
            } else {
                generate_base_equipment(level)
            }
        }
        EquipmentTypes::Shield(_, _) => {
            let possible_base_items = SHIELDS_BY_LEVEL.get(&level);
            let base_item_result = choose_base_item(possible_base_items);
            if let Ok(base_item) = base_item_result {
                BaseEquipment::Shield(base_item)
            } else {
                generate_base_equipment(level)
            }
        }
    }
}
