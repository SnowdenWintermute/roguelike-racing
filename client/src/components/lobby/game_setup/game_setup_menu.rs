use crate::components::common_components::atoms::button_basic::ButtonBasic;
use crate::components::websocket_manager::send_client_input::send_client_input;
use crate::store::websocket_store::WebsocketStore;
use common::packets::client_to_server::PlayerInputs;
use yew::prelude::*;
use yewdux::prelude::use_store;

#[function_component(GameSetupMenu)]
pub fn game_setup_menu() -> Html {
    let (websocket_state, _) = use_store::<WebsocketStore>();
    let leave_game = Callback::from(move |_| {
        send_client_input(&websocket_state.websocket, PlayerInputs::LeaveGame)
    });

    let (websocket_state, _) = use_store::<WebsocketStore>();
    let toggle_ready = Callback::from(move |_| {
        send_client_input(&websocket_state.websocket, PlayerInputs::ToggleReady)
    });

    html!(
            <section class="w-full bg-slate-700 border border-slate-400 p-4 mb-4 flex justify-between">
                <ButtonBasic onclick={leave_game} >
                    {"Leave Game"}
                </ButtonBasic>
                <ButtonBasic onclick={toggle_ready} >{ "Ready" }</ButtonBasic>
            </section>
    )
}
