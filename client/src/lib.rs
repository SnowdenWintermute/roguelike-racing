mod components;
mod router;
mod store;
use crate::components::molocules::display_auth::DisplayAuth;
use crate::components::organisms::websocket_provider::WebsocketProvider;
use crate::{
    components::{
        atoms::main_title::{Color, MainTitle},
        molocules::{
            custom_form::{CustomForm, CustomFormData},
            login_form::LoginForm,
        },
    },
    router::{switch, Route},
};
use gloo::console::log;
use std::ops::Deref;
use yew::prelude::*;
use yew::ContextProvider;
use yew_router::prelude::*;

#[derive(Debug)]
struct MyObject {
    name: String,
    number: u32,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct User {
    pub username: String,
    pub fav: String,
}

#[function_component(App)]
pub fn app() -> Html {
    let user_state = use_state(User::default);
    let main_title_load = Callback::from(|message: String| {
        log!(message);
    });

    use_effect(|| || {});

    let name = "ay";
    log!(name, "asasasasa");
    let my_object = MyObject {
        name: "ay".to_string(),
        number: 10,
    };

    log!(format!("{:#?}", my_object));
    let class = "my_title";
    // let message = Some("amessage");
    let message: Option<&str> = None;
    let tasks = vec!["record video", "butter teeth", "undo", "redo"];

    let custom_form_submit_handler = {
        let user_state = user_state.clone();
        Callback::from(move |data: CustomFormData| {
            log!("submat");
            let mut user = user_state.deref().clone();
            user.username = data.username;
            user.fav = data.fav;
            user_state.set(user);
        })
    };

    let user = User {
        username: "Mike".to_string(),
        fav: "ts".to_string(),
    };

    html! {
        <ContextProvider<User> context={user_state.deref().clone()}>
            <WebsocketProvider >
            <div></div>
            </WebsocketProvider>
        <main class="h-screen text-amber-100 bg-slate-800">
        <LoginForm/>
        <DisplayAuth />
            <MainTitle title="hi there" color={Color::Ok} on_load={main_title_load} />
            <CustomForm submit_handler={custom_form_submit_handler} />

            if let Some(message) = message {
                <p>{message}</p>
            }
            if class == "my_titles" {
                <p>{"tytles"}</p>
            } else {
                <p>{"non"}</p>
            }
            <ul>
                {tasks.iter().map(|task|
                    html!{<li>{task}</li>}).collect::<Html>()}
            </ul>
                <BrowserRouter>
                    <Switch<Route> render={switch} />
                </BrowserRouter>
        </main>
        </ContextProvider<User>>
    }
}
