use common::packets::client_to_server::PlayerInputs;
use web_sys::WebSocket;

pub fn send_client_input(websocket_option: &Option<WebSocket>, player_action: PlayerInputs) {
    match websocket_option {
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
    }
}
