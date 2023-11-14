use common::items::equipment::{EquipmentProperties, EquipmentTypes };
use yew::prelude::*;
use crate::components::game::tabbed_display::item_details_tab::equipment_details::equipment_durability::EquipmentDurability;
mod combat_attributes;
mod equipment_durability;
mod requirements;
mod traits;
mod weapon_damage;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub equipment_properties: EquipmentProperties,
}

#[function_component(EquipmentDetails)]
pub fn equipment_details(props: &Props) -> Html {
    let armor_category_if_any = armor_category(&props.equipment_properties.equipment_type);
    let armor_class_if_any = armor_class(&props.equipment_properties.equipment_type);
    let weapon_damage_if_any =
        weapon_damage::weapon_damage(&props.equipment_properties.equipment_type);

    html!(
            <div>
                <div class="" >
                    {format!("{}", props.equipment_properties.equipment_type)}
                    {armor_category_if_any}
                </div>
                {armor_class_if_any}
                {weapon_damage_if_any}
                <EquipmentDurability
                    durability_option={props.equipment_properties.durability.clone()}
                    equipment_type={props.equipment_properties.equipment_type.clone()}
                />
                {combat_attributes::combat_attributes(&props.equipment_properties)}
                {traits::traits(&props.equipment_properties.traits)}
                {requirements::requirements(&props.equipment_properties.requirements)}

            </div>
    )
}

fn armor_category(equipment_type: &EquipmentTypes) -> String {
    match equipment_type {
        EquipmentTypes::BodyArmor(_, properties) | EquipmentTypes::HeadGear(_, properties) => {
            format!(" ({})", properties.armor_category)
        }
        _ => String::from(""),
    }
}
fn armor_class(equipment_type: &EquipmentTypes) -> Html {
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
