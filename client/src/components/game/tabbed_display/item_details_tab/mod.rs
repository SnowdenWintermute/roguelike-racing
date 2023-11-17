mod equipment_details;
use common::items::Item;
use yew::prelude::*;
use yewdux::prelude::use_store;

use crate::{
    components::game::tabbed_display::item_details_tab::equipment_details::EquipmentDetails,
    store::{
        game_store::{set_compared_item, GameStore},
        ui_store::UIStore,
    },
};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub item: Item,
}

#[function_component(ItemDetailsTab)]
pub fn item_details_tab(props: &Props) -> Html {
    let (game_state, game_dispatch) = use_store::<GameStore>();
    let (ui_state, _) = use_store::<UIStore>();
    let item_id = props.item.entity_properties.id;
    let compared_item = &game_state.compared_item;

    let cloned_game_dispatch = game_dispatch.clone();
    let considered_item_id = props.item.entity_properties.id;
    use_effect_with((ui_state.mod_key_held, item_id), move |_| {
        let cloned_game_dispatch_ii = cloned_game_dispatch.clone();
        set_compared_item(
            cloned_game_dispatch,
            considered_item_id,
            ui_state.mod_key_held,
        );
        move || {
            cloned_game_dispatch_ii.reduce_mut(|store| store.compared_slot = None);
        }
    });

    let display = match &props.item.item_properties {
        common::items::ItemProperties::Consumable(_) => html!({ "Consumable item" }),
        common::items::ItemProperties::Equipment(properties) => {
            html!(<EquipmentDetails equipment_properties={properties.clone()} />)
        }
    };

    let compared_item_name = match &compared_item {
        Some(item) => &item.entity_properties.name,
        None => "",
    };

    let compared_display_option = match &compared_item {
        Some(compared_item) => match &compared_item.item_properties {
            common::items::ItemProperties::Consumable(_) => None,
            common::items::ItemProperties::Equipment(properties) => {
                Some(html!(<EquipmentDetails equipment_properties={properties.clone()} />))
            }
        },
        None => None,
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
            if let Some(compared_display) = compared_display_option {
                {compared_item_name}
                {compared_display}
                <div class="opacity-50 fill-slate-400 h-40 absolute bottom-5 right-3">
                    <img src="public/img/equipment-icons/1h-sword-a.svg" class="h-40 filter" />
                </div>
            }
            </div>
        </div>
    )
}
