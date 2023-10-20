mod components;
mod router;
mod store;
use crate::{
    components::{organisms::websocket_manager::WebsocketManager, pages::lobby::Lobby},
    router::{switch, Route},
};
use yew::prelude::*;
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
    html! {
        <div >
            <WebsocketManager server_url={"ws://127.0.0.1:8081/ws"} />
            <Lobby />
            // <BrowserRouter>
            //     <Switch<Route> render={switch} />
            // </BrowserRouter>
        </div>
    }
}
