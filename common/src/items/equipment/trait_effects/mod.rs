pub mod get_weapon_percent_damage_increase_trait_damage_modifier;
use crate::items::equipment::EquipmentTraits;

pub fn get_armor_class_percentage_increase_trait_ac_modifier(
    equipment_traits: &Option<Vec<EquipmentTraits>>,
) -> f32 {
    if let Some(eq_traits) = equipment_traits {
        for equipment_trait in eq_traits {
            match equipment_trait {
                EquipmentTraits::ArmorClassPercentage(percentage) => {
                    return 1.0 + *percentage as f32 / 100.0
                }
                _ => continue,
            }
        }
    }
    1.0
}

pub fn get_trait_modified_armor_class(
    armor_class: u8,
    equipment_traits: &Option<Vec<EquipmentTraits>>,
) -> u8 {
    armor_class * get_armor_class_percentage_increase_trait_ac_modifier(equipment_traits) as u8
}
