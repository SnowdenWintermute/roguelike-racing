use crate::components::common_components::atoms::{
    button_basic::ButtonBasic, text_input::TextInput,
};
use std::ops::Deref;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub input_placeholder: String,
    pub input_name: String,
    pub submit_handler_callback: Callback<AttrValue>,
    pub button_title: String,
    pub submit_disabled: bool,
}

#[function_component(TextSubmit)]
pub fn text_submit(props: &Props) -> Html {
    let input_state = use_state(|| AttrValue::from(""));
    let handle_input_change = {
        let input_state = input_state.clone();
        Callback::from(move |new_value| input_state.set(new_value))
    };

    let submit_handler_callback = props.submit_handler_callback.clone();
    let handle_submit = {
        let input_state = input_state.clone();
        move |e: SubmitEvent| {
            e.prevent_default();
            let data = input_state.deref().clone();
            submit_handler_callback.emit(data);
        }
    };

    html!(
        <form class="flex mb-2" onsubmit={handle_submit} >
            <TextInput
                name={props.input_name.clone()}
                placeholder={props.input_placeholder.clone()}
                handle_change={handle_input_change}
            />
            <ButtonBasic disabled={props.submit_disabled} extra_styles="border-l-0 " button_type="submit">
                { props.button_title.clone() }
            </ButtonBasic>
        </form>
    )
}
