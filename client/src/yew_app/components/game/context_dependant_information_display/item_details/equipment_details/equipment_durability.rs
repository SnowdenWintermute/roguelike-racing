use common::{items::equipment::EquipmentTypes, primatives::MaxAndCurrent};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub durability_option: Option<MaxAndCurrent<u8>>,
    pub equipment_type: EquipmentTypes,
}

#[function_component(EquipmentDurability)]
pub fn equipment_durability(props: &Props) -> Html {
    match &props.durability_option {
        Some(durability) => {
            html!(<div>{"Durability: "}{durability.current}{"/"}{durability.max}</div>)
        }
        None => {
            if props.equipment_type != EquipmentTypes::Amulet
                && props.equipment_type != EquipmentTypes::Ring
            {
                html!(<div>{"Indestructable"}</div>)
            } else {
                html!()
            }
        }
    }
}
