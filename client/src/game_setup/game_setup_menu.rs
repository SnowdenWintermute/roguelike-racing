use crate::{
    common_components::button_basic::ButtonBasic,
    websocket_provider::send_client_input::send_client_input,
};
use common::game::player_actions::PlayerInputs;
use leptos::*;
use web_sys::{MouseEvent, WebSocket};

#[component]
pub fn game_setup_menu() -> impl IntoView {
    let ws = expect_context::<ReadSignal<Option<WebSocket>>>();

    let leave_game = move |e: MouseEvent| {
        e.prevent_default();
        send_client_input(ws, PlayerInputs::LeaveGame)
    };

    view! {
        <section class="w-full bg-slate-700 border border-slate-400 p-4 mb-4 flex justify-between">
            <ButtonBasic on:click=leave_game>
                "Leave Game"
            </ButtonBasic>
            <ButtonBasic>"Ready"</ButtonBasic>
        </section>
    }
}
