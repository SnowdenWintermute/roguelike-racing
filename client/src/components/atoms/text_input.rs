use gloo::console::log;
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub name: String,
    pub handle_change: Callback<String>,
}

#[function_component(TextInput)]
pub fn text_input(props: &Props) -> Html {
    let handle_change = props.handle_change.clone();
    let on_change = Callback::from(move |event: Event| {
        let target_element = event.target().unwrap();
        let input = target_element.unchecked_into::<HtmlInputElement>();
        log!(input.value());
        handle_change.emit(input.value());
    });

    html!(
        <input
        class="text-black block p-2 mb-2"
        type="text" placeholder={props.name.clone()} name={props.name.clone()} onchange={on_change} />
    )
}
