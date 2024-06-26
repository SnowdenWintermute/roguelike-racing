use crate::yew_app::store::game_store::select_item;
use crate::yew_app::store::game_store::set_item_hovered;
use crate::yew_app::store::game_store::GameStore;
use common::combatants::combat_attributes::CombatAttributes;
use common::items::equipment::EquipmentSlots;
use common::items::Item;
use std::collections::HashMap;
use std::ops::Deref;
use yew::prelude::*;
use yewdux::prelude::use_store;

#[derive(Properties, Eq, PartialEq)]
pub struct Props {
    pub item_option: Option<Item>,
    pub slot: EquipmentSlots,
    pub character_attributes: HashMap<CombatAttributes, u16>,
    pub class: String,
}

#[function_component(PaperDollSlot)]
pub fn paper_doll_slot(props: &Props) -> Html {
    let Props {
        item_option,
        slot,
        class,
        character_attributes,
    } = props;
    let (game_state, game_dispatch) = use_store::<GameStore>();
    let highlighted_class_state = use_state(|| "".to_string());

    let item_display = match item_option {
        Some(item) => item.entity_properties.name.clone(),
        None => "".to_string(),
    };
    let detailed_entity_id_option = match &game_state.detailed_entity {
        Some(detailed_entity) => Some(detailed_entity.get_id()).clone(),
        None => None,
    };
    let hovered_entity_id_option = match &game_state.hovered_entity {
        Some(hovered_entity) => Some(hovered_entity.get_id()).clone(),
        None => None,
    };

    // HANDLE BORDER/BG STYLES FOR COMPARED/FOCUSED/HOVERED EQUIPMENT SLOTS
    let cloned_highlighted_class_state = highlighted_class_state.clone();
    let cloned_compared_slot = game_state.compared_slot.clone();
    let cloned_slot = slot.clone();
    let cloned_item_in_slot_option = item_option.clone();
    let cloned_character_attributes = character_attributes.clone();
    use_effect_with(
        (
            cloned_compared_slot,
            detailed_entity_id_option,
            cloned_character_attributes,
            cloned_item_in_slot_option,
        ),
        move |(compared_slot, _, cloned_character_attributes, cloned_item_in_slot_option)| {
            let mut bg_class = "";

            let mut equipped_item_is_usable = true;
            if let Some(item) = cloned_item_in_slot_option {
                equipped_item_is_usable =
                    item.requirements_satisfied_by_attributes(cloned_character_attributes);
            }

            if !equipped_item_is_usable {
                bg_class = "bg-red-800 opacity-50";
            }
            if let Some(compared_slot) = &compared_slot {
                if *compared_slot == cloned_slot {
                    if game_state.considered_item_unmet_requirements.is_some() {
                        bg_class = "bg-red-800 opacity-50";
                    } else {
                        bg_class = "bg-slate-800";
                    }
                }
            }

            if let Some(detailed_entity_id) = detailed_entity_id_option {
                if let Some(item_in_slot) = &cloned_item_in_slot_option {
                    if item_in_slot.entity_properties.id == detailed_entity_id {
                        cloned_highlighted_class_state.set("border-yellow-400".to_string());
                        cloned_highlighted_class_state
                            .set(format!("{} {}", bg_class, "border-yellow-400"));
                        return;
                    }
                }
            }

            if let Some(hovered_entity_id) = hovered_entity_id_option {
                if let Some(item_in_slot) = &cloned_item_in_slot_option {
                    if item_in_slot.entity_properties.id == hovered_entity_id {
                        cloned_highlighted_class_state
                            .set(format!("{} {}", bg_class, "border-white"));
                        return;
                    }
                }
            }

            cloned_highlighted_class_state.set(format!("{} {}", bg_class, "border-slate-400"));
        },
    );

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

    if props.item_option.is_none() {
        return html!(
            <button class={format!("overflow-ellipsis overflow-hidden border {} {}", class, highlighted_class_state.deref())}>
                {item_display}
            </button>
        );
    }

    html!(
        <button class={format!("overflow-ellipsis overflow-hidden border {} {}",class, highlighted_class_state.deref())}
            onmouseenter={handle_mouse_over_item}
            onmouseleave={handle_mouse_leave_item}
            onfocus={handle_focus_item}
            onblur={handle_blur_item}
            onclick={handle_click}
        >
            {item_display}
        </button>
    )
}
