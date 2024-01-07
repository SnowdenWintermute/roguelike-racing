use crate::store::websocket_store::WebsocketStore;
use common::packets::server_to_client::WebsocketChannelAndUserPacket;
use common::packets::server_to_client::WebsocketChannelFullState;
use common::packets::WebsocketChannelNamespace;
use yewdux::Dispatch;

pub fn handle_websocket_channels_full_update(
    websocket_dispatch: Dispatch<WebsocketStore>,
    packet: WebsocketChannelFullState,
) {
    websocket_dispatch
        .clone()
        .reduce_mut(|store| match packet.namespace {
            WebsocketChannelNamespace::Lobby | WebsocketChannelNamespace::Game => {
                store.websocket_channels.main = packet;
            }
            WebsocketChannelNamespace::Party => store.websocket_channels.party = Some(packet),
            WebsocketChannelNamespace::Chat => store.websocket_channels.chat = Some(packet),
        });
}

pub fn handle_user_joined_websocket_channel(
    websocket_dispatch: Dispatch<WebsocketStore>,
    packet: WebsocketChannelAndUserPacket,
) {
    websocket_dispatch
        .clone()
        .reduce_mut(|store| match packet.channel_namespace {
            WebsocketChannelNamespace::Lobby | WebsocketChannelNamespace::Game => {
                store
                    .websocket_channels
                    .main
                    .usernames_in_channel
                    .insert(packet.username);
            }
            WebsocketChannelNamespace::Party => {
                if let Some(party_channel) = &mut store.websocket_channels.party {
                    party_channel.usernames_in_channel.insert(packet.username);
                }
            }
            WebsocketChannelNamespace::Chat => {
                if let Some(chat_channel) = &mut store.websocket_channels.chat {
                    chat_channel.usernames_in_channel.insert(packet.username);
                }
            }
        });
}

pub fn handle_user_left_websocket_channel(
    websocket_dispatch: Dispatch<WebsocketStore>,
    packet: WebsocketChannelAndUserPacket,
) {
    websocket_dispatch
        .clone()
        .reduce_mut(|store| match packet.channel_namespace {
            WebsocketChannelNamespace::Lobby | WebsocketChannelNamespace::Game => {
                store
                    .websocket_channels
                    .main
                    .usernames_in_channel
                    .remove(&packet.username);
            }
            WebsocketChannelNamespace::Party => {
                if let Some(party_channel) = &mut store.websocket_channels.party {
                    party_channel.usernames_in_channel.remove(&packet.username);
                }
            }
            WebsocketChannelNamespace::Chat => {
                if let Some(chat_channel) = &mut store.websocket_channels.chat {
                    chat_channel.usernames_in_channel.remove(&packet.username);
                }
            }
        });
}
