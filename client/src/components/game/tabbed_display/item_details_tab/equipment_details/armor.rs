use common::items::equipment::{
    trait_effects::get_trait_modified_armor_class, EquipmentTraits, EquipmentTypes,
};
use yew::{html, Html};

pub fn armor_category(equipment_type: &EquipmentTypes) -> String {
    match equipment_type {
        EquipmentTypes::BodyArmor(_, properties) | EquipmentTypes::HeadGear(_, properties) => {
            format!(" ({})", properties.armor_category)
        }
        _ => String::from(""),
    }
}
pub fn armor_class(
    equipment_type: &EquipmentTypes,
    equipment_traits: &Option<Vec<EquipmentTraits>>,
) -> Html {
    let armor_class_option = match equipment_type {
        EquipmentTypes::BodyArmor(_, properties) | EquipmentTypes::HeadGear(_, properties) => {
            Some(properties.armor_class)
        }
        EquipmentTypes::Shield(_, properties) => Some(properties.armor_class),
        _ => None,
    };

    let mut has_modified_ac = false;
    let armor_class_text_option = match armor_class_option {
        Some(armor_class) => {
            let modified_ac = get_trait_modified_armor_class(armor_class, equipment_traits);
            if modified_ac != armor_class {
                has_modified_ac = true
            }
            Some(format!("Armor Class: {}", modified_ac))
        }
        None => None,
    };

    let modified_ac_style = match has_modified_ac {
        true => "text-blue-600",
        false => "",
    };

    if let Some(ac) = armor_class_text_option {
        html!(
        <div class={modified_ac_style} >
            {ac}
        </div>
        )
    } else {
        html!()
    }
}
