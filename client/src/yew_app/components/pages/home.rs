use crate::yew_app::router::Route;
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(Home)]
pub fn home() -> Html {
    html!(
        <div>
            <h1>{"home page"}</h1>
            <div>
            <Link<Route> classes="text-blue-300" to={Route::Hello}>{"link to hello"} </Link<Route>>
            </div>
        </div>
    )
}
