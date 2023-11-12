use crate::components::common_components::atoms::button_blank::ButtonBlank;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub number: usize,
    #[prop_or(Callback::from(|_e:MouseEvent|()))]
    pub click_handler: Callback<MouseEvent>,
    #[prop_or(Callback::from(|_e:FocusEvent|()))]
    pub focus_handler: Callback<FocusEvent>,
    #[prop_or(Callback::from(|_e:MouseEvent|()))]
    pub mouse_enter_handler: Callback<MouseEvent>,
    #[prop_or(Callback::from(|_e:MouseEvent|()))]
    pub mouse_leave_handler: Callback<MouseEvent>,
    #[prop_or(Callback::from(|_e:FocusEvent|()))]
    pub blur_handler: Callback<FocusEvent>,
    #[prop_or(String::from(""))]
    pub button_text: String,
}

#[function_component(ActionMenuButton)]
pub fn action_menu_button(props: &Props) -> Html {
    html!(
        <ButtonBlank class="h-10 w-full border-b border-slate-400 flex hover:bg-slate-950"
            onclick={props.click_handler.clone()}
            onmouseenter={props.mouse_enter_handler.clone()}
            onmouseleave={props.mouse_leave_handler.clone()}
            onfocus={props.focus_handler.clone()}
            onblur={props.blur_handler.clone()}
        >
            <span class="h-full w-10 !min-w-[2.5rem] border-r border-slate-400
            flex items-center justify-center mr-2" >
                {props.number}
            </span>
            <span class="flex-grow h-full flex items-center whitespace-nowrap overflow-hidden overflow-ellipsis" >
                {props.button_text.clone()}
            </span>
        </ButtonBlank>
    )
}
