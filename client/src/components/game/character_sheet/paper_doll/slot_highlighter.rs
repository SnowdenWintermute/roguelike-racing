use crate::store::{game_store::GameStore, ui_store::UIStore};
use common::items::equipment::EquipmentSlots;
use gloo::console::log;
use yew::prelude::*;
use yewdux::prelude::use_store;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub equiped_item_id_option: Option<u32>,
    pub slot: EquipmentSlots,
    pub highlight_class_state: UseStateHandle<String>,
}

#[function_component(SlotHighlighter)]
pub fn slot_highlighter(props: &Props) -> Html {
    let (game_state, _) = use_store::<GameStore>();
    let (ui_state, _) = use_store::<UIStore>();
    let Props {
        equiped_item_id_option,
        slot,
        highlight_class_state,
    } = props;

    let compared_item_id_and_slots_option = match &game_state.compared_item {
        Some(compared_item) => match &compared_item.item_properties {
            common::items::ItemProperties::Consumable(_) => None,
            common::items::ItemProperties::Equipment(equipment_properties) => Some((
                compared_item.entity_properties.id,
                equipment_properties.get_equippable_slots(),
            )),
        },
        None => None,
    };

    let detailed_entity_id_option = match &game_state.detailed_entity {
        Some(detailed_entity) => Some(detailed_entity.get_id()),
        None => None,
    };

    let cloned_highlighted_class_state = highlight_class_state.clone();
    let cloned_compared_item_id_and_slots_option = compared_item_id_and_slots_option.clone();
    let cloned_ui_state = ui_state.clone();
    let cloned_slot = slot.clone();
    let cloned_equiped_item_id_option = equiped_item_id_option.clone();
    use_effect_with(
        (
            cloned_equiped_item_id_option,
            compared_item_id_and_slots_option,
            detailed_entity_id_option,
            ui_state.mod_key_held,
        ),
        move |_| {
            // if comparing
            if let Some(compared_item_id_and_slots) = cloned_compared_item_id_and_slots_option {}
            cloned_highlighted_class_state.set("border-slate-400".to_string());
        },
    );

    html!()
}
