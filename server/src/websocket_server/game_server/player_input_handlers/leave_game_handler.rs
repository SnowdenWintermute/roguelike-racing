use crate::websocket_server::game_server::GameServer;
use common::{
    consts::MAIN_CHAT_ROOM,
    game::{player_actions::GameCreation, RoguelikeRacerPlayer},
    packets::server_to_client::GameServerUpdatePackets,
};

use super::join_room_handler::join_room_handler;

pub fn leave_game_handler(game_server: &mut GameServer, actor_id: usize) {
    let connected_user = match game_server.sessions.get_mut(&actor_id) {
        Some(user) => user,
        None => {
            println!("tried to leave game but no user was found");
            return;
        }
    };

    match &connected_user.current_game_name {
        Some(game_name) => {
            match game_server.games.get_mut(game_name) {
                Some(game) => {
                    // remove them from game and delete their player characters
                    game.partyless_players
                        .remove(&connected_user.username.to_string());
                    for (_, party) in game.adventuring_parties.iter_mut() {
                        let player_option =
                            party.players.remove(&connected_user.username.to_string());
                        if player_option.is_some() {
                            let player = player_option.expect("is some");
                            // delete their characters
                            if player.character_ids.is_some() {
                                for id in player.character_ids.expect("is some") {
                                    party.player_characters.remove(&id);
                                }
                            };
                        }
                    }
                    // if game empty remove it
                    if game.get_number_of_players() == 0 {
                        game_server.games.remove(game_name);
                    }
                }
                None => {
                    println!("no game by that name was found");
                    return;
                }
            }
            // remove game name from user's current game slot
            connected_user.current_game_name = None;
        }

        None => {
            println!("can't leave a game if user isn't in one");
            return;
        }
    }
    // join them to the main channel
    join_room_handler(game_server, MAIN_CHAT_ROOM, actor_id);
    //
    game_server.send_packet(&GameServerUpdatePackets::GameFullUpdate(None), actor_id);
}
