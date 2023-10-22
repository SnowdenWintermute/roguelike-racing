mod components;
mod router;
mod store;
use crate::components::alerts::alert_manager::AlertManager;
use crate::components::lobby::game_setup::GameSetup;
use crate::components::lobby::Lobby;
use crate::components::websocket_manager::WebsocketManager;
use crate::store::game_store::GameStore;
use yew::prelude::*;
use yewdux::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    let (game_state, _) = use_store::<GameStore>();

    html! {
        <div >
            <WebsocketManager server_url={"ws://127.0.0.1:8081/ws"} />
            <AlertManager />
            if game_state.game.is_some() {
                <GameSetup />
            } else {
                <Lobby />
            }
        </div>
    }
}
