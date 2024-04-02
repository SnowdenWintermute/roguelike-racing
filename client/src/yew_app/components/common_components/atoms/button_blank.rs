use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub id: String,
    #[prop_or(Callback::from(|_e:MouseEvent|()))]
    pub onclick: Callback<MouseEvent>,
    #[prop_or(Callback::from(|_e:MouseEvent|()))]
    pub onmouseenter: Callback<MouseEvent>,
    #[prop_or(Callback::from(|_e:MouseEvent|()))]
    pub onmouseleave: Callback<MouseEvent>,
    #[prop_or(Callback::from(|_e:FocusEvent|()))]
    pub onfocus: Callback<FocusEvent>,
    #[prop_or(Callback::from(|_e:FocusEvent|()))]
    pub onblur: Callback<FocusEvent>,
    #[prop_or(false)]
    pub disabled: bool,
    #[prop_or_default]
    pub class: String,
    #[prop_or("button".to_string())]
    pub button_type: String,
    pub children: Html,
}

#[function_component(ButtonBlank)]
pub fn button_blank(props: &Props) -> Html {
    let onclick = props.onclick.clone();
    let button_onclick = Callback::from(move |e: MouseEvent| {
        onclick.emit(e);
    });

    html!(
        <button
            class={props.class.clone()}
            onclick={button_onclick}
            onmouseenter={props.onmouseenter.clone()}
            onmouseleave={props.onmouseleave.clone()}
            onfocus={props.onfocus.clone()}
            onblur={props.onblur.clone()}
            disabled={props.disabled}
            type={props.button_type.clone()}
            id={props.id.clone()}
        >
            {props.children.clone()}
        </button>

    )
}
