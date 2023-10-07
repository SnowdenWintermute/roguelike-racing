use common::packets::server_to_client::GameServerUpdatePackets;

use crate::websocket_server::game_server::GameServer;

pub fn adventuring_party_creation_request_handler(
    game_server: &mut GameServer,
    actor_id: &u32,
    party_name: String,
) {
    let connected_user = match game_server.sessions.get_mut(actor_id) {
        Some(user) => user,
        None => {
            println!("received a request to create an adventuring party 
                     but the actor_id did not correspond to any user registered with the game server");
            return;
        }
    };

    // find out what game they're in
    let current_game_name = match connected_user.current_game_name.clone() {
        Some(name) => name.clone(),
        None => {
            println!("tried to create an adventuring party but the player didn't have a current_game reference");
            return;
        }
    };

    let game = match game_server.games.get_mut(&current_game_name) {
        Some(game) => game,
        None => {
            println!(
            "tried to create an adventuring party but the player's game reference didn't correspond
                     to any game on the server"
        );
            return;
        }
    };

    if game
        .partyless_players
        .get(&connected_user.username)
        .is_none()
    {
        println!(
            "tried to create an adventuring party but no partyless_players found by user's name"
        );
        return;
    }

    // create the party in the game (use a game impl function so the client can reuse this part)
    let party_id = game.add_adventuring_party(party_name);
    // add them to the party
    game.put_player_in_adventuring_party(party_id, connected_user.username.clone());

    let party = match game.adventuring_parties.get(&party_id) {
        Some(party) => party.clone(),
        None => {
            println!("failed to create party");
            return;
        }
    };

    println!("creating party: {:#?}", party.clone());

    // update the game room with the new party
    game_server.emit_packet(
        &current_game_name,
        &GameServerUpdatePackets::AdventuringPartyCreated(party.clone()),
        None,
    );
}
