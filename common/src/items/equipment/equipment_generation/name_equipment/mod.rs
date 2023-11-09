mod prefix_names;
pub mod suffix_names;
use self::{prefix_names::get_prefix_name, suffix_names::get_suffix_name};
use crate::items::equipment::EquipmentProperties;

pub fn name_equipment(equipment_properties: &EquipmentProperties) -> String {
    let mut output = String::new();
    let mut prefix_name = "";
    let mut suffix_name = "";
    let base_item_name = match equipment_properties.equipment_type {
        crate::items::equipment::EquipmentTypes::BodyArmor(base_item, _) => {
            format!("{}", base_item)
        }
        crate::items::equipment::EquipmentTypes::HeadGear(base_item, _) => format!("{}", base_item),
        crate::items::equipment::EquipmentTypes::Ring => format!("Ring"),
        crate::items::equipment::EquipmentTypes::Amulet => format!("Amulet"),
        crate::items::equipment::EquipmentTypes::OneHandedMeleeWeapon(base_item, _) => {
            format!("{}", base_item)
        }
        crate::items::equipment::EquipmentTypes::TwoHandedMeleeWeapon(base_item, _) => {
            format!("{}", base_item)
        }
        crate::items::equipment::EquipmentTypes::TwoHandedRangedWeapon(base_item, _) => {
            format!("{}", base_item)
        }
        crate::items::equipment::EquipmentTypes::Shield(base_item, _) => format!("{}", base_item),
    };

    for affix in &equipment_properties.affixes {
        match affix {
            crate::items::equipment::affixes::Affix::Prefix(prefix_type, tier) => {
                prefix_name = get_prefix_name(prefix_type, tier);
            }
            crate::items::equipment::affixes::Affix::Suffix(suffix_type, tier) => {
                suffix_name = get_suffix_name(suffix_type, tier);
            }
        }
    }

    output.push_str(prefix_name);
    output.push_str(" ");
    output.push_str(&base_item_name);
    output.push_str(" of ");
    output.push_str(suffix_name);

    output
}
