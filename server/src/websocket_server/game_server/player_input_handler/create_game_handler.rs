use common::game::player_actions::{GameCreation, PlayerInputs};

use crate::websocket_server::game_server::GameServer;

pub fn create_game_handler(game_server: &mut GameServer, message_content: GameCreation) {
    println!("game creation request received");
    println!("{:#?}", message_content)

    // check if game name exists
    // create the game
    // put a reference to the creator's actor address in the games list of those
    // put a reference to the current game in session
}
