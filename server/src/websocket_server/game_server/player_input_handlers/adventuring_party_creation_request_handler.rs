use common::{
    errors::AppError,
    packets::server_to_client::{AdventuringPartyCreation, GameServerUpdatePackets},
};

use crate::websocket_server::game_server::GameServer;

pub fn adventuring_party_creation_request_handler(
    game_server: &mut GameServer,
    actor_id: &u32,
    party_name: String,
) -> Result<(), AppError> {
    let connected_user = game_server.sessions.get_mut(actor_id).ok_or(AppError {
        error_type: common::errors::AppErrorTypes::ServerError,
        message: "No user found".to_string(),
    })?;

    let username = connected_user.username.clone();

    // find out what game they're in
    let current_game_name = connected_user.current_game_name.clone().ok_or(AppError {
        error_type: common::errors::AppErrorTypes::ServerError,
        message: "Missing reference to current game".to_string(),
    })?;

    let game = game_server
        .games
        .get_mut(&current_game_name)
        .ok_or(AppError {
            error_type: common::errors::AppErrorTypes::ServerError,
            message: "No game found".to_string(),
        })?;

    // should only be able to create a party if they aren't in one yet
    let _partyless_players =
        game.partyless_players
            .get(&connected_user.username)
            .ok_or(AppError {
                error_type: common::errors::AppErrorTypes::ServerError,
                message: "Leave your current party if you wish to create a new one".to_string(),
            })?;

    // create the party in the game (use a game impl function so the client can reuse this part)
    let party_id = game.add_adventuring_party(party_name);
    // add them to the party
    game.put_player_in_adventuring_party(party_id, connected_user.username.clone());

    let party = game.adventuring_parties.get(&party_id).ok_or(AppError {
        error_type: common::errors::AppErrorTypes::ServerError,
        message: "Something went wrong while trying to create your new party".to_string(),
    })?;
    let party_to_send = party.clone();

    println!("creating party: {:#?}", &party_to_send);

    // tell the client what party they are in
    game_server.send_packet(
        &GameServerUpdatePackets::ClientAdventuringPartyId(Some(party_id)),
        *actor_id,
    );
    // update the game room with the new party
    let creation_data = AdventuringPartyCreation {
        party: party_to_send,
        username_created_by: username,
    };
    game_server.emit_packet(
        &current_game_name,
        &GameServerUpdatePackets::AdventuringPartyCreated(creation_data),
        None,
    );
    Ok(())
}
