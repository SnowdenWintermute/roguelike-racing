use crate::websocket_server::game_server::GameServer;
use common::{errors::AppError, packets::server_to_client::GameServerUpdatePackets};

impl GameServer {
    pub fn game_list_update_request_handler(&mut self, actor_id: u32) -> Result<(), AppError> {
        let game_list = self.create_game_list_update();
        println!("client requested game list");
        self.send_packet(&GameServerUpdatePackets::GameList(game_list), actor_id)
    }
}
