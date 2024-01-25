pub mod determine_action_button_text;
use super::build_action_button_properties::ActionMenuButtonProperties;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub number: usize,
    pub properties: ActionMenuButtonProperties,
}

#[function_component(ActionMenuButton)]
pub fn action_menu_button(props: &Props) -> Html {
    html!(
        <button class="h-10 w-full border-b border-slate-400 flex hover:bg-slate-950 disabled:opacity-50"
            onclick={props.properties.click_handler.clone()}
            onmouseenter={props.properties.mouse_enter_handler.clone()}
            onmouseleave={props.properties.mouse_leave_handler.clone()}
            onfocus={props.properties.focus_handler.clone()}
            onblur={props.properties.blur_handler.clone()}
            disabled={props.properties.should_be_disabled}
        >
            <span class="h-full w-10 !min-w-[2.5rem] border-r border-slate-400
            flex items-center justify-center mr-2" >
                {props.number}
            </span>
            <span class="flex-grow h-full flex items-center whitespace-nowrap overflow-hidden overflow-ellipsis" >
                {props.properties.text.clone()}
            </span>
        </button>
    )
}
