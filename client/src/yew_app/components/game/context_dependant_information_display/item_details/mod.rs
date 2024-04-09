mod equipment_details;
mod requirements;
mod unmet_requirements_calculator;
use crate::yew_app::components::game::context_dependant_information_display::item_details::equipment_details::EquipmentDetails;
use crate::yew_app::components::game::tailwind_class_loader::SPACING_REM;
use crate::yew_app::components::game::tailwind_class_loader::SPACING_REM_SMALL;
use crate::yew_app::store::game_store::set_compared_item;
use crate::yew_app::store::game_store::GameStore;
use crate::yew_app::store::ui_store::UIStore;
use common::combat::combat_actions::CombatAction;
use common::items::equipment::EquipmentSlots;
use common::items::equipment::EquipmentTypes;
use common::items::Item;
use common::items::ItemProperties;
use std::rc::Rc;
use yew::prelude::*;
use yewdux::prelude::use_store;

use super::action_details_context_info::ActionDetailsContextInfo;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub item: Item,
}

#[function_component(ItemDetails)]
pub fn item_details(props: &Props) -> Html {
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
        ItemProperties::Consumable(_) => {
            let consumable_combat_action =
                CombatAction::ConsumableUsed(props.item.entity_properties.id);
            html!(<ActionDetailsContextInfo combat_action={consumable_combat_action} hide_title={true} />)
        }
        ItemProperties::Equipment(properties) => html!(
        <EquipmentDetails
              equipment_properties={properties.clone()}
              requirements={props.item.requirements.clone()}
              entity_id={item_id}
              is_compared_item={false}
        />
              ),
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

    let compared_display_hide_class = if compared_display_option.is_none() {
        "opacity-0 h-0 pointer-events-none"
    } else {
        ""
    };

    html!(
        <div class="flex-grow flex">
            // ITEM
            <div class="border border-slate-400 bg-slate-700 min-h-20 max-h-[13.375rem] max-w-1/2 relative overflow-y-auto"
                style={format!("margin-right: {}rem; width: 50%; padding: {}rem", SPACING_REM_SMALL / 2.0, SPACING_REM)}
            >
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
            // COMPARED
            <div class={ format!("border border-slate-400 bg-slate-700 min-h-20 max-h-[13.375rem] max-w-1/2 relative overflow-y-auto {compared_display_hide_class}" )}
                style={format!("margin-left: {}rem; width: 50%; padding: {}rem", SPACING_REM_SMALL / 2.0, SPACING_REM)}
            >
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
