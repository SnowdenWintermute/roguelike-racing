use std::ops::Deref;

use gloo::console::log;
use yew::prelude::*;

use crate::{
    components::atoms::{custom_button::CustomButton, text_input::TextInput},
    User,
};

#[derive(Properties, PartialEq)]
pub struct Props {
    // pub name: String,
    pub submit_handler: Callback<CustomFormData>,
}

#[derive(Default, Clone)]
pub struct CustomFormData {
    pub username: String,
    pub fav: String,
    pub count: u32,
}

#[function_component(CustomForm)]
pub fn custom_form(props: &Props) -> Html {
    let user_state = use_state(|| User::default());
    let state = use_state(|| CustomFormData::default());
    let user_context: Option<User> = use_context();

    let cloned_state = state.clone();
    let handle_change = Callback::from(move |username| {
        cloned_state.set(CustomFormData {
            username,
            ..cloned_state.deref().clone()
        });
    });

    let cloned_state = state.clone();
    let handle_fav_change = Callback::from(move |fav| {
        cloned_state.set(CustomFormData {
            fav,
            ..cloned_state.deref().clone()
        });
    });

    let cloned_state = state.clone();
    let handle_click = Callback::from(move |_| {
        let mut data = cloned_state.deref().clone();
        data.count += 1;
        cloned_state.set(data)
    });

    let submit_handler = props.submit_handler.clone();
    let cloned_state = state.clone();
    let handle_submit = move |e: SubmitEvent| {
        e.prevent_default();
        let data = cloned_state.deref().clone();
        submit_handler.emit(data);
    };

    html!(
        <form onsubmit={handle_submit}>
            <TextInput name="username" handle_change={handle_change} />
            <TextInput name="favorite" handle_change={handle_fav_change} />
            <CustomButton title="Submit" onclick={handle_click} />
            <p>{"Username: "}{&state.username}</p>
            <p>{state.count}</p>
            <p>{"Username from context: "}{user_context.clone().unwrap_or_default().username}</p>
        </form>
    )
}
