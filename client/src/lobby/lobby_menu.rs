use crate::websocket_provider::send_client_input::send_client_input;
use common::adventuring_party::AdventuringParty;
use common::game::player_actions::{GameCreation, PlayerInputRequest, PlayerInputs};
use common::game::RoguelikeRacerGame;
use common::packets::server_to_client::RoguelikeRacerAppState;
use leptos::ev::InputEvent;
use leptos::*;
use web_sys::WebSocket;

#[component]
pub fn lobby_menu(cx: Scope) -> impl IntoView {
    let game = expect_context::<RwSignal<RoguelikeRacerAppState>>(cx);
    let ws = expect_context::<ReadSignal<Option<WebSocket>>>(cx);

    let (new_game_name, set_new_game_name) = create_signal(cx, "".to_string());

    let create_game = move |_| {
        send_client_input(
            ws,
            PlayerInputs::CreateGame(GameCreation {
                name: new_game_name.get(),
                password: None,
            }),
        )
    };

    // let leave_game = move |_| send_client_input(ws, PlayerInputs::LeaveGame);
    // <li>
    //     <button on:click=leave_game value="">"Leave Game " { new_game_name }</button>
    // </li>
    view! { cx,
        <section class="bg-slate-700 p-4 mb-4 flex">
             <input type="text"
             class="bg-slate-700 border border-sky-500 h-10 p-4"
            on:input=move |ev| {
                set_new_game_name(event_target_value(&ev));
            }
            prop:value=new_game_name
            prop:placeholder="Enter a game name..."
        />
        <button class="border border-l-0 border-sky-500 h-10 cursor-pointer pr-4 pl-4
        flex justify-center items-center disabled:opacity-50 disabled:cursor-auto"
            prop:disabled={move || new_game_name.get().len() < 1}
            on:click=create_game>"Create Game"</button>
        </section>
    }
}
