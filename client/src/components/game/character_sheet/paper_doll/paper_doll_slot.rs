use common::items::Item;
use yew::prelude::*;
use yewdux::prelude::{use_store, Dispatch};
use crate::store::game_store::{DetailableEntities, GameStore};

#[derive(Properties, Eq, PartialEq)]
pub struct Props {
    pub item_option: Option<Item>,
    pub class: String,
}

#[function_component(PaperDollSlot)]
pub fn paper_doll_slot(props: &Props) -> Html {
    let Props { item_option, class } = props;
    let (_, game_dispatch) = use_store::<GameStore>();

    let item_display = match item_option {
        Some(item) => item.entity_properties.name.clone(),
        None => "".to_string(),
    };

    if props.item_option.is_none() {
        return html!(
            <button class={class}>
                {item_display}
            </button>
        );
    }

    fn set_item_hovered(game_dispatch: Dispatch<GameStore>, item_option: Option<Item>) {
        game_dispatch.reduce_mut(|store| {
            if let Some(item) = item_option {
                let entity_details = DetailableEntities::Item(item.clone());
                store.hovered_entity = Some(entity_details);
            } else {
                store.hovered_entity = None;
            }
        })
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

    html!(
        <button class={class}
            onmouseenter={handle_mouse_over_item}
            onmouseleave={handle_mouse_leave_item}
            onfocus={handle_focus_item}
            onblur={handle_blur_item} >
            {item_display}
        </button>
    )
}
