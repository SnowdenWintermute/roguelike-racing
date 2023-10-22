use common::packets::client_to_server::{GameCreation, PlayerInputs};
use std::ops::Deref;
use yew::prelude::*;
use yewdux::prelude::use_store;

use crate::{
    components::{
        common_components::atoms::{button_basic::ButtonBasic, text_input::TextInput},
        websocket_manager::send_client_input::send_client_input,
    },
    store::websocket_store::WebsocketStore,
};

#[function_component(LobbyMenu)]
pub fn lobby_menu() -> Html {
    let (websocket_state, _) = use_store::<WebsocketStore>();
    let game_name = use_state(|| AttrValue::from(""));

    let handle_change = {
        let game_name = game_name.clone();
        Callback::from(move |new_name| game_name.set(new_name))
    };

    let create_game = move |event: SubmitEvent| {
        event.prevent_default();
        send_client_input(
            &websocket_state.websocket,
            PlayerInputs::CreateGame(GameCreation {
                name: game_name.deref().clone().to_string(),
                password: None,
            }),
        );
    };

    let (websocket_state, _) = use_store::<WebsocketStore>();
    let refresh_game_list = Callback::from(move |_| {
        send_client_input(&websocket_state.websocket, PlayerInputs::RequestGameList)
    });

    html!(
        <section class="w-full bg-slate-700 border border-slate-400 p-4 mb-4 flex justify-between">
            <form class="flex" onsubmit={create_game}>
                <TextInput name="game name" placeholder="Game name..." handle_change={handle_change} />
                <ButtonBasic disabled=false extra_styles="border-l-0 " button_type="submit" >
                    {"Create Game"}
                </ButtonBasic>
            </form>
                <ButtonBasic disabled=false button_type="button" onclick={refresh_game_list} >
                    {"Refresh List"}
                </ButtonBasic>
        </section>
    )
}
