use gloo::console::log;
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub placeholder: String,
    pub name: String,
    pub handle_change: Callback<AttrValue>,
}

#[function_component(TextInput)]
pub fn text_input(props: &Props) -> Html {
    let handle_change = props.handle_change.clone();
    let on_change = Callback::from(move |event: Event| {
        let target_element = event.target().unwrap();
        let input = target_element.unchecked_into::<HtmlInputElement>();
        handle_change.emit(input.value().into());
    });

    html!(
        <input
        class="bg-slate-700 border border-slate-400 h-10 p-4"
        type="text" placeholder={props.placeholder.clone()} name={props.name.clone()} onchange={on_change} />
    )
}
