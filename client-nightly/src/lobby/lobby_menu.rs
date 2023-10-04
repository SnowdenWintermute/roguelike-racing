use crate::{
    common_components::button_basic::ButtonBasic,
    websocket_provider::send_client_input::send_client_input,
};
use common::game::player_actions::{GameCreation, PlayerInputs};
use leptos::*;
use web_sys::{MouseEvent, WebSocket};

#[component]
pub fn lobby_menu() -> impl IntoView {
    let ws = expect_context::<ReadSignal<Option<WebSocket>>>();

    let (new_game_name, set_new_game_name) = create_signal("".to_string());
    let disabled = MaybeSignal::derive(move || new_game_name().len() < 1);

    let create_game = move |_: MouseEvent| {
        send_client_input(
            ws,
            PlayerInputs::CreateGame(GameCreation {
                name: new_game_name(),
                password: None,
            }),
        )
    };

    let refresh_game_list =
        move |_: MouseEvent| send_client_input(ws, PlayerInputs::RequestGameList);

    // let leave_game = move |_| send_client_input(ws, PlayerInputs::LeaveGame);
    // <li>
    //     <button on:click=leave_game value="">"Leave Game " { new_game_name }</button>
    // </li>
    view! {
        <section class="bg-slate-700 p-4 mb-4 flex">
             <input type="text"
             class="bg-slate-700 border border-sky-500 h-10 p-4"
            on:input=move |ev| {
                set_new_game_name(event_target_value(&ev));
            }
            prop:value=new_game_name
            prop:placeholder="Enter a game name..."
        />
        <ButtonBasic
            disabled=disabled
            on:click=create_game
            extra_styles="border-l-0 "
        >
            "Create Game"
        </ButtonBasic>
        <ButtonBasic on:click=refresh_game_list>"Refresh List"</ButtonBasic>
        </section>
    }
}
