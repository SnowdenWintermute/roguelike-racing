use serde::Deserialize;
use serde::Serialize;
pub mod client_to_server;
pub mod server_to_client;

#[derive(Debug, Serialize, Deserialize, Hash, Clone, PartialEq, Default, Eq)]
pub enum WebsocketChannelNamespace {
    #[default]
    Lobby,
    Party,
    Game,
    Chat,
}
