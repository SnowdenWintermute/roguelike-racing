use common::items::Item;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub item: Item,
}

#[function_component(ItemDetailsTab)]
pub fn item_details_tab(props: &Props) -> Html {
    let display = match &props.item.item_properties {
        common::items::ItemProperties::Consumable(_) => html!({ "Consumable item" }),
        common::items::ItemProperties::Equipment(properties) => match properties.durability.clone()
        {
            Some(durability) => {
                html!(
                    <div>{durability.current}{"/"}{durability.max}</div>
                )
            }
            None => html!(<div>{"Indestructable"}</div>),
        },
    };

    html!(
        <div class="h-full w-full">
            {props.item.entity_properties.name.clone()}
            {display}
        </div>
    )
}
