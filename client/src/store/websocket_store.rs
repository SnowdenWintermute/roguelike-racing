use web_sys::WebSocket;
use yewdux::prelude::*;

#[derive(Store, Default, PartialEq, Clone)]
pub struct WebsocketStore {
    pub websocket: Option<WebSocket>,
}
