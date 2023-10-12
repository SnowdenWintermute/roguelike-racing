use crate::websocket_server::game_server::{get_mut_user, GameServer};
use common::errors::AppError;
use common::packets::server_to_client::{AdventuringPartyCreation, GameServerUpdatePackets};

impl GameServer {
    pub fn adventuring_party_creation_request_handler(
        &mut self,
        actor_id: u32,
        party_name: String,
    ) -> Result<(), AppError> {
        let connected_user = get_mut_user(&mut self.sessions, actor_id)?;
        let username = connected_user.username.clone();

        let current_game_name = connected_user.current_game_name.clone().ok_or(AppError {
            error_type: common::errors::AppErrorTypes::ServerError,
            message: "Missing reference to current game".to_string(),
        })?;

        let game = self.games.get_mut(&current_game_name).ok_or(AppError {
            error_type: common::errors::AppErrorTypes::ServerError,
            message: "No game found".to_string(),
        })?;

        let _partyless_players =
            game.partyless_players
                .get(&connected_user.username)
                .ok_or(AppError {
                    error_type: common::errors::AppErrorTypes::ServerError,
                    message: "Leave your current party if you wish to create a new one".to_string(),
                })?;

        let party_id = game.add_adventuring_party(party_name);
        game.put_player_in_adventuring_party(party_id, connected_user.username.clone())?;

        let party = game.adventuring_parties.get(&party_id).ok_or(AppError {
            error_type: common::errors::AppErrorTypes::ServerError,
            message: "Something went wrong while trying to create your new party".to_string(),
        })?;
        let party_to_send = party.clone();

        self.send_packet(
            &GameServerUpdatePackets::ClientAdventuringPartyId(Some(party_id)),
            actor_id,
        )?;
        self.emit_packet(
            &current_game_name,
            &GameServerUpdatePackets::AdventuringPartyCreated(AdventuringPartyCreation {
                party: party_to_send,
                username_created_by: username,
            }),
            None,
        )
    }
}
