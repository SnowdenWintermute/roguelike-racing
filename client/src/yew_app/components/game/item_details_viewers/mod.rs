use crate::yew_app::components::game::context_dependant_information_display::item_details::ItemDetails;
use crate::yew_app::store::game_store::DetailableEntities;
use crate::yew_app::store::game_store::GameStore;
use yew::prelude::*;
use yewdux::use_store;

#[function_component(ItemDetailsAndComparison)]
pub fn item_details_and_comparison() -> Html {
    let (game_state, _) = use_store::<GameStore>();
    let detailed_entity = &game_state.detailed_entity;
    let hovered_entity = &game_state.hovered_entity;

    let hovered_item_option = match hovered_entity {
        Some(detailable) => match detailable {
            DetailableEntities::Item(item) => Some(item.clone()),
            _ => None,
        },
        None => None,
    };
    let detailed_item_option = match detailed_entity {
        Some(detailable) => match detailable {
            DetailableEntities::Item(item) => Some(item.clone()),
            _ => None,
        },
        None => None,
    };

    let item_option = if let Some(hovered_item_details) = hovered_item_option {
        Some(hovered_item_details)
    } else if let Some(detailed_item_details) = detailed_item_option {
        Some(detailed_item_details)
    } else {
        None
    };

    if let Some(item) = item_option {
        html!(
            <ItemDetails item={item} flip_display_order={true} />
        )
    } else {
        html!()
    }
}
