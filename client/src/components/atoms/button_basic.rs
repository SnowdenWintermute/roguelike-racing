use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    #[prop_or(Callback::from(|_|()))]
    pub onclick: Callback<()>,
    #[prop_or(false)]
    pub disabled: bool,
    #[prop_or_default]
    pub extra_styles: String,
    pub button_type: String,
    pub children: Html,
}

#[function_component(ButtonBasic)]
pub fn button_basic(props: &Props) -> Html {
    let onclick = props.onclick.clone();
    let button_onclick = Callback::from(move |_| {
        onclick.emit(());
    });

    html!(
        <button
            class={format!(
                "{} {}",
                "border border-slate-400 h-10 cursor-pointer pr-4 pl-4
                flex justify-center items-center disabled:opacity-50 disabled:cursor-auto",
                props.extra_styles.clone(),
            )}
            onclick={button_onclick}
            disabled={props.disabled}
            type={props.button_type.clone()}
        >
            {props.children.clone()}
        </button>

    )
}
