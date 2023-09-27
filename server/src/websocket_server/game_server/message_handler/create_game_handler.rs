use common::game::player_actions::PlayerInputs;

use crate::websocket_server::game_server::GameServer;

pub fn create_game_handler(game_server: GameServer, message: PlayerInputs){
    match message {
        PlayerInputs::CreateGame(data) => {
            //
        },
        _ => ()
    }
}
