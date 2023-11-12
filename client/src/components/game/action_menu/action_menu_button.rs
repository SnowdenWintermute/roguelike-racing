use crate::components::common_components::atoms::button_blank::ButtonBlank;
use yew::prelude::*;

use super::set_up_actions::ActionMenuButtonProperties;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub number: usize,
    pub properties: ActionMenuButtonProperties,
}

#[function_component(ActionMenuButton)]
pub fn action_menu_button(props: &Props) -> Html {
    html!(
        <ButtonBlank class="h-10 w-full border-b border-slate-400 flex hover:bg-slate-950"
            onclick={props.properties.click_handler.clone()}
            onmouseenter={props.properties.mouse_enter_handler.clone()}
            onmouseleave={props.properties.mouse_leave_handler.clone()}
            onfocus={props.properties.focus_handler.clone()}
            onblur={props.properties.blur_handler.clone()}
        >
            <span class="h-full w-10 !min-w-[2.5rem] border-r border-slate-400
            flex items-center justify-center mr-2" >
                {props.number}
            </span>
            <span class="flex-grow h-full flex items-center whitespace-nowrap overflow-hidden overflow-ellipsis" >
                {props.properties.text.clone()}
            </span>
        </ButtonBlank>
    )
}
