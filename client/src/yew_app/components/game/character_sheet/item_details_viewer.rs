use crate::yew_app::components::game::context_dependant_information_display::item_details::ItemDetails;
use crate::yew_app::components::game::items_on_ground::ItemsOnGround;
use crate::yew_app::components::game::tailwind_class_loader::SPACING_REM;
use crate::yew_app::components::game::tailwind_class_loader::SPACING_REM_SMALL;
use crate::yew_app::store::game_store::DetailableEntities;
use crate::yew_app::store::game_store::GameStore;
use yew::prelude::*;
use yewdux::use_store;

#[function_component(ItemDetailsViewer)]
pub fn item_details_viewer() -> Html {
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

    let item_details_display = if let Some(item) = item_option {
        html!(
            <ItemDetails item={item} flip_display_order={ false } />
        )
    } else {
        html!()
    };

    let viewing_character_sheet = game_state.viewing_inventory
        || game_state.viewing_equipped_items
        || game_state.viewing_attribute_point_assignment_menu;

    html!(
        <div class="flex"
             style={format!("padding-top: {}rem; ", SPACING_REM_SMALL)}
            >
            <div class="min-w-[25rem] max-w-[25rem] h-[13.375rem]"
                 style={format!("margin-right: {}rem; ", SPACING_REM)}
               >
               if viewing_character_sheet {
                   <div class="max-h-[13.375rem]">
                       <ItemsOnGround max_height={13.375} />
                   </div>
               } else {
                   <div class="max-h-[13.375rem]" />
               }
           </div>
           if viewing_character_sheet {
               {item_details_display}
           }
        </div>
    )
}
