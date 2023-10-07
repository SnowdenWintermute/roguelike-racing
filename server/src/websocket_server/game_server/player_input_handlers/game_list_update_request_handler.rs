use common::packets::server_to_client::GameServerUpdatePackets;

use crate::websocket_server::game_server::GameServer;

pub fn game_list_update_request_handler(game_server: &mut GameServer, actor_id: u32) {
    let game_list = game_server.create_game_list_update();
    game_server.send_packet(&GameServerUpdatePackets::GameList(game_list), actor_id)
}
