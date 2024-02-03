mod consumable_details;
mod equipment_details;
mod requirements;
mod unmet_requirements_calculator;
use crate::components::game::tabbed_display::item_details_tab::consumable_details::ConsumableDetails;
use crate::components::game::tabbed_display::item_details_tab::equipment_details::EquipmentDetails;
use crate::store::game_store::set_compared_item;
use crate::store::game_store::GameStore;
use crate::store::ui_store::UIStore;
use common::items::equipment::EquipmentSlots;
use common::items::equipment::EquipmentTypes;
use common::items::Item;
use common::items::ItemProperties;
use std::rc::Rc;
use yew::prelude::*;
use yewdux::prelude::use_store;

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

    let mod_key_tooltip = if should_display_mod_tooltip(&game_state, &props.item) {
        html!(
            <span>
                {" "}
                <span class="border border-slate-400 rounded-md pr-1 pl-1" >{ "shift" }</span>
            </span>
        )
    } else {
        html!()
    };

    let display = match &props.item.item_properties {
        ItemProperties::Consumable(properties) => html!(
        <ConsumableDetails
            consumable_properties={properties.clone()}
            requirements={props.item.requirements.clone()}
            entity_id={item_id}
        />
        ),
        ItemProperties::Equipment(properties) => html!(
        <EquipmentDetails
              equipment_properties={properties.clone()}
              requirements={props.item.requirements.clone()}
              entity_id={item_id}
              is_compared_item={false}
        />
              ),
    };

    let consumable_description_option = match &props.item.item_properties {
        ItemProperties::Consumable(properties) => {
            Some(properties.consumable_type.get_description())
        }
        ItemProperties::Equipment(_) => None,
    };

    let compared_item_name = match &compared_item {
        Some(item) => &item.entity_properties.name,
        None => "",
    };

    let compared_display_option = match props.item.item_properties {
        ItemProperties::Consumable(_) => None,
        ItemProperties::Equipment(_) => match &compared_item {
            Some(compared_item) => match &compared_item.item_properties {
                ItemProperties::Consumable(_) => None,
                ItemProperties::Equipment(properties) => Some(html!(<EquipmentDetails
                      equipment_properties={properties.clone()}
                      requirements={compared_item.requirements.clone()}
                      entity_id={compared_item.entity_properties.id}
                      is_compared_item={true}
                      />)),
            },
            None => None,
        },
    };

    html!(
        <div class="w-full h-full flex">
            <div class="h-full w-1/2 relative">
                <span>
                    {"Item considering"}
                </span>
                <div class="mr-2 mb-1 mt-1 h-[1px] bg-slate-400" />
                {props.item.entity_properties.name.clone()}
                {display.clone()}
                <div class="opacity-50 fill-slate-400 h-40 absolute bottom-5 right-3">
                    <img src="public/img/equipment-icons/1h-sword-a.svg" class="h-40 filter" />
                </div>
            </div>
            <div class="h-full w-1/2 relative">
            if let Some(compared_display) = compared_display_option {
                <span class="flex justify-between pr-2">
                    {"Currently equipped"}
                {mod_key_tooltip}
                </span>
                <div class="mr-2 mb-1 mt-1 h-[1px] bg-slate-400" />
                {compared_item_name}
                {compared_display}
                <div class="opacity-50 fill-slate-400 h-40 absolute bottom-5 right-3">
                    <img src="public/img/equipment-icons/1h-sword-a.svg" class="h-40 filter" />
                </div>
            } else if let Some(consumable_description) = consumable_description_option{
                <div class="flex justify-between pr-2">
                    {"Description"}
                </div>
                <div class="mr-2 mb-1 mt-1 h-[1px] bg-slate-400" />
                <div>
                    {consumable_description}
                </div>
            }
            </div>
        </div>
    )
}

fn should_display_mod_tooltip(game_state: &Rc<GameStore>, equipped_item: &Item) -> bool {
    if game_state.compared_slot == Some(EquipmentSlots::RightRing)
        || game_state.compared_slot == Some(EquipmentSlots::MainHand)
    {
        match &equipped_item.item_properties {
            ItemProperties::Consumable(_) => false,
            ItemProperties::Equipment(equipment_properties) => {
                match equipment_properties.equipment_type {
                    EquipmentTypes::Ring => true,
                    EquipmentTypes::OneHandedMeleeWeapon(_, _) => true,

                    _ => false,
                }
            }
        }
    } else {
        false
    }
}
