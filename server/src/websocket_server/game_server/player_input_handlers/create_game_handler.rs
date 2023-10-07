use crate::websocket_server::game_server::{
    player_input_handlers::join_game_handler::join_game_handler, GameServer,
};
use common::game::player_actions::{GameCreation, PlayerInputs};
use common::game::{RoguelikeRacerGame, RoguelikeRacerPlayer};

pub fn create_game_handler(
    game_server: &mut GameServer,
    actor_id: u32,
    message_content: GameCreation,
) {
    println!("game creation request received");
    let GameCreation {
        name: game_name,
        password: _,
    } = &message_content;

    let connected_user = match game_server.sessions.get_mut(&actor_id) {
        Some(user) => user,
        None => {
            println!("received a request to start a new game but the actor_id did not correspond to any user registered with the game server");
            return;
        }
    };

    println!("{:#?}", message_content);
    // reject if actor already in a game
    if connected_user.current_game_name.is_some() {
        println!("leave your current game before creating one");
        return;
    }

    // check if game name exists
    if game_server.games.get(game_name).is_some() {
        println!("a game with name {} already exists", game_name);
        return;
    }

    // create the game and register it with the game_server
    let new_game = RoguelikeRacerGame::new(game_name.to_string());
    game_server.games.insert(game_name.to_string(), new_game);
    // join game
    let actor_id = connected_user.id;
    join_game_handler(game_server, actor_id, game_name.to_string());
    println!("{:#?}", game_server);
}
