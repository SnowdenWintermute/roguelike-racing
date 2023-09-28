use common::adventuring_party::AdventuringParty;
use common::game::player_actions::{GameCreation, PlayerInputRequest, PlayerInputs};
use common::game::RoguelikeRacerGame;

use leptos::ev::InputEvent;
use leptos::*;
use web_sys::WebSocket;

#[component]
pub fn lobby(cx: Scope) -> impl IntoView {
    let game = expect_context::<RwSignal<Option<RoguelikeRacerGame>>>(cx);
    let ws = expect_context::<ReadSignal<Option<WebSocket>>>(cx);

    let (new_game_name, set_new_game_name) = create_signal(cx, "".to_string());

    let create_game = move |_| {
        ws.with(|socket| match socket {
            Some(ws) => {
                let some_player_action = PlayerInputs::CreateGame(GameCreation {
                    name: new_game_name.get(),
                    password: None,
                });
                let serialized = serde_cbor::to_vec(&some_player_action);

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
    };

    view! { cx,
    <section class="h-full w-full p-2 text-zinc-300" >
        <div class="bg-slate-700 p-2 h-full">
            <ul class="list-none">
                <li>
                    <button on:click=create_game>"Create Game " { new_game_name }</button>
                </li>
            </ul>
             <input type="text"
            on:input=move |ev| {
                set_new_game_name(event_target_value(&ev));
            }
            prop:value=new_game_name
        />
        </div>
    </section>
    }
}
