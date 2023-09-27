// use leptos::*;
// use leptos_meta::BodyContext;
// use wasm_bindgen::JsValue;
// use web_sys::WebSocket;

// #[allow(unused_variables)]
// pub fn get_ws(url: &str) -> Result<WasmOnlyWebsocket, JsValue> {
//     get_ws_inner(url)
// }

// #[derive(Clone, Debug, PartialEq, Eq)]
// pub struct AppWebSocket {
//     ws: Option<WebSocket>,
// }

// impl AppWebSocket {
//     pub fn chat(&self) {
//         if let Some(ws) = &self.ws {
//             ws.send_with_str("Hi from new WS");
//         } else {
//             log!("empty WS");
//         }
//     }

//     pub fn send_binary(&self, data: &[u8]) {
//         if let Some(ws) = &self.ws {
//             ws.send_with_u8_array(data);
//         } else {
//             log!("empty WS");
//         }
//     }
// }

// pub enum WasmOnlyWebsocket {
//     IsWasm(WebSocket),
//     IsNative,
// }

// cfg_if::cfg_if! {
//     if #[cfg(target_arch = "wasm32")] {
//         #[inline]
//         fn get_ws_inner(url: &str) -> Result<WasmOnlyWebsocket, JsValue> {
//             log!("wasm32");
//                 let ws = WebSocket::new(url)?;
//                 Ok(WasmOnlyWebsocket::IsWasm(ws))
//         }
//     } else {
//         #[inline]
//         fn get_ws_inner(_url: &str) -> Result<WasmOnlyWebsocket, JsValue> {
//             log!("non wasm32");
//             Ok(WasmOnlyWebsocket::IsNative)
//         }
//     }
// }
