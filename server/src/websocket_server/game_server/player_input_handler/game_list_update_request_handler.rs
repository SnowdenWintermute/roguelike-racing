use common::packets::server_to_client::GameServerUpdatePackets;

use crate::websocket_server::game_server::GameServer;

pub fn game_list_update_request_handler(game_server: &mut GameServer, actor_id: usize) {
    let connected_user = match game_server.sessions.get_mut(&actor_id) {
        Some(user) => user,
        None => {
            println!("received a request to start a new game but the actor_id did not correspond to any user registered with the game server");
            return;
        }
    };

    let game_list = game_server.create_game_list_update(actor_id);
    game_server.send_packet(GameServerUpdatePackets::GameList(game_list), actor_id)
}
