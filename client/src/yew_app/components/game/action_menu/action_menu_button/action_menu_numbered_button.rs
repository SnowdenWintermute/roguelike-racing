use crate::yew_app::components::game::{
    action_menu::build_action_button_properties::ActionMenuButtonProperties,
    tailwind_class_loader::BUTTON_HEIGHT,
};
use yew::prelude::*;

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
