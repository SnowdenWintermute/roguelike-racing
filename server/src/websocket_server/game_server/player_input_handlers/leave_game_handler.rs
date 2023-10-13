use crate::websocket_server::game_server::{
    getters::{get_mut_game, get_mut_user},
    GameServer,
};
use common::{
    app_consts::{error_messages, MAIN_CHAT_ROOM},
    errors::AppError,
    packets::server_to_client::{GameServerUpdatePackets, PlayerRemovedFromGame},
};

impl GameServer {
    pub fn leave_game_handler(&mut self, actor_id: u32) -> Result<(), AppError> {
        self.join_room_handler(MAIN_CHAT_ROOM, actor_id)?;
        let player_and_game = self.remove_player_from_game(actor_id)?;
        self.emit_packet(
            player_and_game.game_name.as_str(),
            &GameServerUpdatePackets::UserLeftGame(player_and_game.username.clone()),
            Some(actor_id),
        )?;
        self.send_packet(&GameServerUpdatePackets::GameFullUpdate(None), actor_id)?;
        self.send_packet(
            &GameServerUpdatePackets::ClientAdventuringPartyId(None),
            actor_id,
        )
    }

    pub fn remove_player_from_game(
        self: &mut GameServer,
        actor_id: u32,
    ) -> Result<PlayerRemovedFromGame, AppError> {
        let connected_user = get_mut_user(&mut self.sessions, actor_id)?;
        let game_name_leaving = connected_user.current_game_name.clone().ok_or(AppError {
            error_type: common::errors::AppErrorTypes::ServerError,
            message: error_messages::MISSING_GAME_REFERENCE.to_string(),
        })?;
        let game = get_mut_game(&mut self.games, &game_name_leaving)?;
        // remove player from game
        game.partyless_players
            .remove(&connected_user.username.clone());
        game.remove_player_from_adventuring_party(connected_user.username.clone(), false);
        // if game empty remove it
        if game.get_number_of_players() < 1 {
            self.games.remove(&game_name_leaving);
        }

        // remove game name from user's current game slot
        connected_user.current_game_name = None;
        Ok(PlayerRemovedFromGame {
            username: connected_user.username.clone(),
            game_name: game_name_leaving,
        })
    }
}
