use crate::yew_app::components::game::{
    action_menu::{
        build_action_button_properties::ActionMenuButtonProperties, set_keyup_listeners::GameKeys,
    },
    tailwind_class_loader::BUTTON_HEIGHT_SMALL,
};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub properties: ActionMenuButtonProperties,
}

#[function_component(ActionMenuTopButton)]
pub fn action_menu_top_button(props: &Props) -> Html {
    let key_to_show = match &props.properties.dedicated_keys_option {
        Some(keys) => match keys[0] {
            GameKeys::Cancel => "Esc",
            GameKeys::Confirm => "R",
            GameKeys::Next => "E",
            GameKeys::Previous => "W",
            GameKeys::S => "S",
            GameKeys::I => "I",
            GameKeys::D => "D",
            GameKeys::O => "O",
            GameKeys::F => "F",
            GameKeys::P => "P",
            GameKeys::T => "T",
        },
        None => "",
    };

    html!(
        <button
            class="w-full border border-slate-400 bg-slate-700
                   flex hover:bg-slate-950 disabled:opacity-50 max-w-fit whitespace-nowrap text-ellipsis overflow-hidden
                   mr-2 last:mr-0"
                style={format!("height: {}rem; ", BUTTON_HEIGHT_SMALL)}
            onclick={props.properties.click_handler.clone()}
            onmouseenter={props.properties.mouse_enter_handler.clone()}
            onmouseleave={props.properties.mouse_leave_handler.clone()}
            onfocus={props.properties.focus_handler.clone()}
            onblur={props.properties.blur_handler.clone()}
            disabled={props.properties.should_be_disabled}
        >
            <span class="flex-grow h-full flex items-center whitespace-nowrap overflow-hidden overflow-ellipsis pr-2 pl-2 " >
                {props.properties.text.clone()}
                {format!(" ({key_to_show})")}
            </span>
        </button>
    )
}
