use yew::prelude::*;
use yew_router::prelude::*;

use crate::router::Route;

#[function_component(Hello)]
pub fn hello() -> Html {
    let navigator = use_navigator().unwrap();
    let handle_click = Callback::from(move |_| navigator.push(&Route::Home));

    html!(
    <div>
        <h1>{"hello world"}</h1>
        <button onclick={handle_click} >{"go home"}</button>
    </div>
    )
}
