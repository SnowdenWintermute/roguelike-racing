use common::adventuring_party::AdventuringParty;
use common::game::player_actions::{GameCreation, PlayerInputRequest, PlayerInputs};
use common::game::RoguelikeRacerGame;

use leptos::ev::InputEvent;
use leptos::*;
use web_sys::WebSocket;

pub fn send_client_input(ws: ReadSignal<Option<WebSocket>>, player_action: PlayerInputs) {
    ws.with(|socket| match socket {
        Some(ws) => {
            let serialized = serde_cbor::to_vec(&player_action);
            match serialized {
                Ok(bytes) => ws.send_with_u8_array(bytes.as_slice()),
                Err(_) => Ok(()),
            };
        }
        None => {
            println!("no websocket in global state");
            ()
        }
    });
    ()
}

#[component]
pub fn lobby(cx: Scope) -> impl IntoView {
    // let game = expect_context::<RwSignal<Option<RoguelikeRacerGame>>>(cx);
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

    let leave_game = move |_| send_client_input(ws, PlayerInputs::LeaveGame);
    let join_game = move |e| send_client_input(ws, PlayerInputs::JoinGame(event_target_value(&e)));

    view! { cx,
    <main class="h-screen w-screen p-2 bg-teal-950 text-zinc-300" >
        <section class="bg-slate-700 p-2 h-full">
            <ul class="list-none">
                <li>
                    <button on:click=create_game>"Create Game " { new_game_name }</button>
                </li>
                <li>
                    <button on:click=join_game value="">"Join Game " { new_game_name }</button>
                </li>
                <li>
                    <button on:click=leave_game value="">"Leave Game " { new_game_name }</button>
                </li>
            </ul>
             <input type="text"
            on:input=move |ev| {
                set_new_game_name(event_target_value(&ev));
            }
            prop:value=new_game_name
        />
        </section>
    </main>
    }
}
