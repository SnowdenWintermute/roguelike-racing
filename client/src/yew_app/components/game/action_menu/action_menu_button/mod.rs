mod action_menu_top_button;
pub mod determine_action_button_text;
use self::action_menu_top_button::ActionMenuTopButton;
use super::{
    build_action_button_properties::ActionMenuButtonProperties, ActionButtonPropertiesByCategory,
};
use crate::yew_app::components::game::{
    action_menu::set_keyup_listeners::GameKeys, tailwind_class_loader::BUTTON_HEIGHT,
};
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
        match &button_properties.dedicated_key_option {
            Some(dedicated_key) => match dedicated_key {
                GameKeys::Cancel
                | GameKeys::Confirm
                | GameKeys::KeysSI
                | GameKeys::KeysDO
                | GameKeys::KeysFP => top_buttons.push(html!(<ActionMenuTopButton 
                                properties={button_properties.clone()}
                                dedicated_key={dedicated_key.clone()}
                                />)),
                GameKeys::Next | GameKeys::Previous => next_prev_buttons.push(html!(
                                <ActionMenuTopButton 
                                   properties={button_properties.clone()}
                                   dedicated_key={dedicated_key.clone()}
                                   />)),
            },
            None => (),
        }
    }

    (top_buttons, numbered_buttons, next_prev_buttons)
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub number: i32,
    pub properties: ActionMenuButtonProperties,
}

#[function_component(ActionMenuNumberedButton)]
pub fn action_menu_numbered_button(props: &Props) -> Html {
    let key_to_show = props.number.clone().to_string();

    html!(
        <button
            class="w-full border-b border-r border-l first:border-t border-slate-400 bg-slate-700 flex hover:bg-slate-950 disabled:opacity-50"
                style={format!("height: {}rem; ", BUTTON_HEIGHT)}
            onclick={props.properties.click_handler.clone()}
            onmouseenter={props.properties.mouse_enter_handler.clone()}
            onmouseleave={props.properties.mouse_leave_handler.clone()}
            onfocus={props.properties.focus_handler.clone()}
            onblur={props.properties.blur_handler.clone()}
            disabled={props.properties.should_be_disabled}
        >
            <span class="h-full w-10 !min-w-[2.5rem] border-r border-slate-400
            flex items-center justify-center mr-2" >
                {key_to_show}
            </span>
            <span class="flex-grow h-full flex items-center whitespace-nowrap overflow-hidden overflow-ellipsis" >
                {props.properties.text.clone()}
            </span>
        </button>
    )
}
