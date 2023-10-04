use crate::websocket_server::game_server::GameServer;
use common::game::{player_actions::GameCreation, RoguelikeRacerPlayer};

pub fn join_game_handler(game_server: &mut GameServer, actor_id: usize, game_name: String) {
    let game = match game_server.games.get_mut(&game_name) {
        Some(game) => game,
        None => {
            println!("no game by that name was found");
            return;
        }
    };
    let connected_user = match game_server.sessions.get_mut(&actor_id) {
        Some(user) => user,
        None => {
            println!("received a request to join a game but the actor_id did not correspond to any user registered with the game server");
            return;
        }
    };

    // @TODO
    // reject if game is full
    // reject if game has started
    if game.time_started.is_some() {
        println!("can't join a game that has already started");
        return;
    }

    // reject if actor already in a game
    if connected_user.current_game_name.is_some() {
        println!("leave your current game before joining another one");
        return;
    }

    // put the client's actor_id in the game
    let new_player = RoguelikeRacerPlayer::new(Some(actor_id), connected_user.username.to_string());
    game.partyless_players
        .insert(connected_user.username.to_string(), new_player);
    // put a reference to the current game in connected_user
    connected_user.current_game_name = Some(game_name.to_string());

    // join them to the "room" for the game
    // send update to them and their roommates
}
