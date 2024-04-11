mod action_menu_change_target_button;
mod action_menu_numbered_button;
mod action_menu_top_button;
pub mod determine_action_button_text;
use self::{
    action_menu_change_target_button::ActionMenuChangeTargetButton,
    action_menu_numbered_button::ActionMenuNumberedButton,
    action_menu_top_button::ActionMenuTopButton,
};
use super::build_action_button_properties::ActionMenuButtonProperties;
use crate::yew_app::components::game::action_menu::set_keyup_listeners::GameKeys;
use yew::prelude::*;

pub fn create_action_menu_buttons(
    top_button_properties: &Vec<ActionMenuButtonProperties>,
    numbered_button_properties_on_current_page: &Vec<ActionMenuButtonProperties>,
    next_prev_button_properties: &Vec<ActionMenuButtonProperties>,
) -> (Vec<Html>, Vec<Html>, Vec<Html>) {
    let mut last_assigned_button_number = 0;
    let mut numbered_buttons = vec![];
    let mut top_buttons = vec![];
    let mut next_prev_buttons = vec![];

    for button_properties in numbered_button_properties_on_current_page.iter() {
        last_assigned_button_number += 1;
        Some(last_assigned_button_number);
        numbered_buttons.push(html!(
            <ActionMenuNumberedButton
            properties={button_properties.clone()}
            number={last_assigned_button_number}
            />
        ));
    }

    for button_properties in top_button_properties.iter() {
        match &button_properties.category {
            super::set_keyup_listeners::ActionButtonCategories::Top => {
                top_buttons.push(html!(<ActionMenuTopButton 
                                properties={button_properties.clone()}
                                />))
            }
            super::set_keyup_listeners::ActionButtonCategories::Numbered => (),
            super::set_keyup_listeners::ActionButtonCategories::NextPrevious => next_prev_buttons
                .push(html!(
                                <ActionMenuTopButton 
                                   properties={button_properties.clone()}
                                   />)),
        }
    }

    for button_properties in next_prev_button_properties.iter() {
        match &button_properties.dedicated_keys_option {
            Some(dedicated_keys) => {
                for key in dedicated_keys {
                    match key {
                        GameKeys::Next | GameKeys::Previous => next_prev_buttons.push(html!(
                        <ActionMenuChangeTargetButton
                            properties={button_properties.clone()}
                            dedicated_key={key.clone()}
                        />
                        )),
                        _ => (),
                    }
                }
            }
            None => todo!(),
        }
    }

    (top_buttons, numbered_buttons, next_prev_buttons)
}
