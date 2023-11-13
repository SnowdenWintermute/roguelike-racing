mod equipment_details;

use common::items::Item;
use yew::prelude::*;

use crate::components::game::tabbed_display::item_details_tab::equipment_details::EquipmentDetails;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub item: Item,
}

#[function_component(ItemDetailsTab)]
pub fn item_details_tab(props: &Props) -> Html {
    let display = match &props.item.item_properties {
        common::items::ItemProperties::Consumable(_) => html!({ "Consumable item" }),
        common::items::ItemProperties::Equipment(properties) => {
            html!(<EquipmentDetails equipment_properties={properties.clone()} />)
        }
    };

    html!(
        <div class="h-full w-full">
            {props.item.entity_properties.name.clone()}
            {display}
        </div>
    )
}
