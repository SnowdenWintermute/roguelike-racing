use common::{
    app_consts::error_messages,
    errors::AppError,
    game::getters::get_mut_player,
    packets::server_to_client::{GameServerUpdatePackets, PlayerAdventuringPartyChange},
};

use crate::websocket_server::game_server::{
    getters::{get_mut_game, get_mut_user},
    GameServer,
};

impl GameServer {
    pub fn join_party_handler(&mut self, actor_id: u32, party_id: u32) -> Result<(), AppError> {
        let connected_user = get_mut_user(&mut self.sessions, actor_id)?;
        let username = connected_user.username.clone();
        let current_game_name =
            connected_user
                .current_game_name
                .clone()
                .ok_or_else(|| AppError {
                    error_type: common::errors::AppErrorTypes::ServerError,
                    message: error_messages::MISSING_GAME_REFERENCE.to_string(),
                })?;
        let game = get_mut_game(&mut self.games, &current_game_name)?;
        let player = get_mut_player(game, username.clone())?;

        if player.party_id.is_some() {
            return Err(AppError {
                error_type: common::errors::AppErrorTypes::InvalidInput,
                message: error_messages::ALREADY_IN_PARTY.to_string(),
            });
        }

        game.put_player_in_adventuring_party(party_id, username.clone())?;

        self.send_packet(
            &GameServerUpdatePackets::ClientAdventuringPartyId(Some(party_id)),
            actor_id,
        )?;

        self.emit_packet(
            &current_game_name,
            &GameServerUpdatePackets::PlayerChangedAdventuringParty(PlayerAdventuringPartyChange {
                username,
                party_id: Some(party_id),
            }),
            None,
        )
    }
}
