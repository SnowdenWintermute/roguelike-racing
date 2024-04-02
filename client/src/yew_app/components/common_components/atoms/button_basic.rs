use crate::yew_app::components::common_components::atoms::button_blank::ButtonBlank;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    #[prop_or(Callback::from(|_e: MouseEvent|()))]
    pub onclick: Callback<MouseEvent>,
    #[prop_or(Callback::from(|_e: FocusEvent|()))]
    pub onfocus: Callback<FocusEvent>,
    #[prop_or(Callback::from(|_e: FocusEvent|()))]
    pub onblur: Callback<FocusEvent>,
    #[prop_or(false)]
    pub disabled: bool,
    #[prop_or_default]
    pub extra_styles: String,
    #[prop_or("button".to_string())]
    pub button_type: String,
    pub children: Html,
}

#[function_component(ButtonBasic)]
pub fn button_basic(props: &Props) -> Html {
    html!(
        <ButtonBlank
            class={
                format!(
                    "border border-slate-400 h-10 cursor-pointer pr-4 pl-4
                    flex justify-center items-center disabled:opacity-50 disabled:cursor-auto {}",
                    props.extra_styles.clone(),
                )
            }
            onclick={props.onclick.clone()}
            onfocus={props.onfocus.clone()}
            onblur={props.onblur.clone()}
            disabled={props.disabled}
            button_type={props.button_type.clone()}
        >
            {props.children.clone()}
        </ButtonBlank>
    )
}
