mod components;
mod router;
mod store;
use crate::components::alerts::alert_manager::AlertManager;
use crate::components::game::Game;
use crate::components::lobby::game_setup::GameSetup;
use crate::components::lobby::Lobby;
use crate::components::websocket_manager::WebsocketManager;
use crate::store::game_store::GameStore;
use gloo::console::log;
use gloo::events::EventListener;
use gloo::utils::window;
use wasm_bindgen::{JsCast, UnwrapThrowExt};
use web_sys::HtmlElement;
use yew::prelude::*;
use yewdux::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    let (game_state, _) = use_store::<GameStore>();
    let game = game_state.game.clone();
    let keyup_listener_state = use_state(|| None::<EventListener>);
    let click_listener_state = use_state(|| None::<EventListener>);
    //
    use_effect_with((), move |_| {
        let listener = EventListener::new(&window(), "keyup", |event| {
            let event = event.dyn_ref::<web_sys::KeyboardEvent>().unwrap_throw();
            log!(&format!("{}", event.key()))
        });
        keyup_listener_state.set(Some(listener));
    });

    use_effect_with((), move |_| {
        let listener = EventListener::new(&window(), "click", |event| {
            let event = event.dyn_ref::<web_sys::MouseEvent>().unwrap_throw();
            let target = event.target();
            if let Some(target) = target {
                let element = target.unchecked_into::<HtmlElement>();
                log!(&format!("{:#?}", element.id()));
            }
        });
        click_listener_state.set(Some(listener));
    });

    html! {
        <div >
            <WebsocketManager server_url={"ws://127.0.0.1:8081/ws"} />
            <AlertManager />
            if game_state.game.is_some() && game.unwrap().time_started.is_some() {
                <Game />
            }else if game_state.game.is_some() {
                <GameSetup />
            } else {
                <Lobby />
            }
        </div>
    }
}
