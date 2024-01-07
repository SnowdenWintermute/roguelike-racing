use common::packets::server_to_client::WebsocketChannelsState;
use web_sys::WebSocket;
use yewdux::prelude::*;

#[derive(Store, Default, PartialEq, Clone)]
pub struct WebsocketStore {
    pub websocket: Option<WebSocket>,
    pub websocket_channels: WebsocketChannelsState,
}
