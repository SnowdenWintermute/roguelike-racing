use crate::{
    common_components::button_basic::ButtonBasic, home_page::ClientPartyId,
    websocket_provider::send_client_input::send_client_input,
};
use common::game::{player_actions::PlayerInputs, RoguelikeRacerGame};
use leptos::*;
use web_sys::{MouseEvent, WebSocket};

#[component]
pub fn game_setup_menu() -> impl IntoView {
    let ws = expect_context::<ReadSignal<Option<WebSocket>>>();
    // let game = expect_context::<RwSignal<Option<RoguelikeRacerGame>>>();
    // let party_id = expect_context::<RwSignal<ClientPartyId>>();

    let leave_game = move |_e: MouseEvent| {
        send_client_input(ws, PlayerInputs::LeaveGame);
        // game.update(move |game_state| *game_state = None);
        // party_id.update(move |party_id_state| *party_id_state = ClientPartyId(None));
    };

    view! {
        <section class="w-full bg-slate-700 border border-slate-400 p-4 mb-4 flex justify-between">
            <ButtonBasic on:click=leave_game>"Leave Game"</ButtonBasic>
            <ButtonBasic>"Ready"</ButtonBasic>
        </section>
    }
}
