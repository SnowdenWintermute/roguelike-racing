use yew::prelude::*;
use yewdux::use_store;

use crate::store::game_store::GameStore;

#[derive(Properties, Eq, PartialEq)]
pub struct Props {}

#[function_component(ActionMenuPageManager)]
pub fn action_menu_page_manager(_: &Props) -> Html {
    // let (game_state, game_dispatch) = use_store::<GameStore>();

    // let cloned_current_page_number = game_state.action_menu_current_page_number.clone();
    // let cloned_action_button_properties = action_button_properties.clone();
    // let cloned_button_props_on_current_page = button_props_on_current_page.clone();
    // use_effect_with(
    //     (
    //         game_state.action_menu_current_page_number,
    //         action_button_properties.clone(),
    //     ),
    //     move |_| {
    //         let min_index = cloned_current_page_number * PAGE_SIZE;
    //         let max_index = cloned_current_page_number * PAGE_SIZE + PAGE_SIZE - 1;
    //         let filtered_actions = cloned_action_button_properties
    //             .deref()
    //             .iter()
    //             .enumerate()
    //             .filter(|(i, _)| *i as u8 >= min_index && *i as u8 <= max_index)
    //             .map(|(_, item)| item.clone())
    //             .collect::<Vec<ActionMenuButtonProperties>>();
    //         cloned_button_props_on_current_page.set(filtered_actions);
    //     },
    // );
    html!()
}
