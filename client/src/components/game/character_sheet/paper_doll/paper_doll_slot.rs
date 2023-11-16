use crate::{
    components::game::character_sheet::paper_doll::slot_highlighter::SlotHighlighter,
    store::{
        game_store::{select_item, set_item_hovered, GameStore},
        ui_store::UIStore,
    },
};
use common::items::{equipment::EquipmentSlots, Item};
use gloo::console::log;
use std::ops::Deref;
use yew::prelude::*;
use yewdux::prelude::use_store;

#[derive(Properties, Eq, PartialEq)]
pub struct Props {
    pub item_option: Option<Item>,
    pub slot: EquipmentSlots,
    pub class: String,
}

#[function_component(PaperDollSlot)]
pub fn paper_doll_slot(props: &Props) -> Html {
    let Props {
        item_option,
        slot,
        class,
    } = props;
    let (game_state, game_dispatch) = use_store::<GameStore>();
    let (ui_state, _) = use_store::<UIStore>();
    let highlighted_class_state = use_state(|| "".to_string());

    let item_display = match item_option {
        Some(item) => item.entity_properties.name.clone(),
        None => "".to_string(),
    };

    let item_id_option = match item_option {
        Some(item) => Some(item.entity_properties.id),
        None => None,
    };

    if props.item_option.is_none() {
        return html!(
            <button class={class}>
                <SlotHighlighter
                    equiped_item_id_option={item_id_option}
                    slot={slot.clone()}
                    highlight_class_state={highlighted_class_state}
                />
                {item_display}
            </button>
        );
    }

    let cloned_dispatch = game_dispatch.clone();
    let cloned_item_option = props.item_option.clone();
    let handle_mouse_over_item = Callback::from(move |_| {
        let cloned_dispatch = cloned_dispatch.clone();
        let cloned_item_option = cloned_item_option.clone();
        set_item_hovered(cloned_dispatch, cloned_item_option)
    });
    let cloned_dispatch = game_dispatch.clone();
    let handle_mouse_leave_item = Callback::from(move |_| {
        let cloned_dispatch = cloned_dispatch.clone();
        set_item_hovered(cloned_dispatch, None)
    });
    let cloned_dispatch = game_dispatch.clone();
    let cloned_item_option = props.item_option.clone();
    let handle_focus_item = Callback::from(move |_| {
        let cloned_dispatch = cloned_dispatch.clone();
        let cloned_item_option = cloned_item_option.clone();
        set_item_hovered(cloned_dispatch, cloned_item_option)
    });
    let cloned_dispatch = game_dispatch.clone();
    let handle_blur_item = Callback::from(move |_| {
        let cloned_dispatch = cloned_dispatch.clone();
        set_item_hovered(cloned_dispatch, None)
    });
    let cloned_dispatch = game_dispatch.clone();
    let cloned_item_option = props.item_option.clone();
    let handle_click = Callback::from(move |_| {
        let cloned_item_option = cloned_item_option.clone();
        let cloned_dispatch = cloned_dispatch.clone();
        select_item(cloned_dispatch, cloned_item_option);
    });

    html!(
        <button class={format!("overflow-ellipsis overflow-hidden border {} {}",class, highlighted_class_state.deref())}
            onmouseenter={handle_mouse_over_item}
            onmouseleave={handle_mouse_leave_item}
            onfocus={handle_focus_item}
            onblur={handle_blur_item}
            onclick={handle_click}
        >
            <SlotHighlighter
                equiped_item_id_option={item_id_option}
                slot={slot.clone()}
                highlight_class_state={highlighted_class_state.clone()}
            />
            {item_display}
        </button>
    )
}
