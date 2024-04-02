use crate::yew_app::components::alerts::alert_manager::AlertManager;
use crate::yew_app::components::game::Game;
use crate::yew_app::components::global_keyboard_event_manager::GlobalKeyboardEventManager;
use crate::yew_app::components::lobby::game_setup::GameSetup;
use crate::yew_app::components::lobby::Lobby;
use crate::yew_app::components::tooltips::TooltipManager;
use crate::yew_app::components::websocket_manager::WebsocketManager;
use crate::yew_app::store::game_store::GameStore;

use super::Props;
use yew::prelude::*;
use yewdux::prelude::*;

#[function_component(YewApp)]
pub fn app(props: &Props) -> Html {
    let (game_state, _) = use_store::<GameStore>();
    let game = game_state.game.clone();
    let in_production = std::env!("TRUNK_PROD");
    // log!(format!("in production: {in_production}"));
    let websocket_server_url = if in_production == "true" {
        "wss://roguelikeracing.com/ws"
    } else {
        // "wss://roguelikeracing.com/ws"
        "ws://127.0.0.1:8082/ws"
    };

    html! {
        <div class="relative" >
            <GlobalKeyboardEventManager />
            <WebsocketManager server_url={websocket_server_url} />
            <AlertManager />
            <TooltipManager />
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
