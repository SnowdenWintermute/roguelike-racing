mod components;
mod router;
mod store;
use crate::components::alerts::alert_manager::AlertManager;
use crate::components::game::Game;
use crate::components::global_keyboard_event_manager::GlobalKeyboardEventManager;
use crate::components::lobby::game_setup::GameSetup;
use crate::components::lobby::Lobby;
use crate::components::websocket_manager::WebsocketManager;
use crate::store::game_store::GameStore;
use yew::prelude::*;
use yewdux::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    let (game_state, _) = use_store::<GameStore>();
    let game = game_state.game.clone();

    html! {
        <div >
            <GlobalKeyboardEventManager />
            <WebsocketManager server_url={"ws://127.0.0.1:8082/ws"} />
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
