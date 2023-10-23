// use crate::components::pages::{hello::Hello, home::Home};
// use yew::prelude::*;
use yew_router::Routable;

#[derive(Routable, Debug, Clone, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/hello")]
    Hello,
}

// pub fn switch(route: Route) -> Html {
//     match route {
//         Route::Home => html!(<Home/>),
//         Route::Hello => html!(<Hello/>),
//     }
// }
