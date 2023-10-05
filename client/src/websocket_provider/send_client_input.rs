use common::game::player_actions::PlayerInputs;
use leptos::*;
use web_sys::WebSocket;

pub fn send_client_input(ws: ReadSignal<Option<WebSocket>>, player_action: PlayerInputs) {
    ws.with(|socket| match socket {
        Some(ws) => {
            let serialized = serde_cbor::to_vec(&player_action);
            let _ = match serialized {
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
