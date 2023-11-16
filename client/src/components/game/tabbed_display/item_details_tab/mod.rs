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
        <div class="w-full h-full flex">
            <div class="h-full w-1/2 relative">
                {props.item.entity_properties.name.clone()}
                {display.clone()}
                <div class="opacity-50 fill-slate-400 h-40 absolute bottom-5 right-3">
                    <img src="public/img/equipment-icons/1h-sword-a.svg" class="h-40 filter" />
                </div>
            </div>
            <div class="h-full w-1/2 relative">
                {props.item.entity_properties.name.clone()}
                {display}
                <div class="opacity-50 fill-slate-400 h-40 absolute bottom-5 right-3">
                    <img src="public/img/equipment-icons/1h-sword-a.svg" class="h-40 filter" />
                </div>
            </div>
        </div>
    )
}
