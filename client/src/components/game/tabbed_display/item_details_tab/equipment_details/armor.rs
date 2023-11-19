use common::items::equipment::EquipmentTypes;
use yew::{html, Html};

pub fn armor_category(equipment_type: &EquipmentTypes) -> String {
    match equipment_type {
        EquipmentTypes::BodyArmor(_, properties) | EquipmentTypes::HeadGear(_, properties) => {
            format!(" ({})", properties.armor_category)
        }
        _ => String::from(""),
    }
}
pub fn armor_class(equipment_type: &EquipmentTypes) -> Html {
    let armor_class_option = match equipment_type {
        EquipmentTypes::BodyArmor(_, properties) | EquipmentTypes::HeadGear(_, properties) => {
            Some(format!("Armor Class: {}", properties.armor_class))
        }
        EquipmentTypes::Shield(_, properties) => {
            Some(format!("Armor Class: {}", properties.armor_class))
        }
        _ => None,
    };
    if let Some(ac) = armor_class_option {
        html!(
        <div>
            {ac}
        </div>
        )
    } else {
        html!()
    }
}
