use super::join_room_handler::join_room_handler;
use crate::websocket_server::game_server::GameServer;
use common::{
    consts::MAIN_CHAT_ROOM,
    errors::AppError,
    game::{player_actions::GameCreation, RoguelikeRacerPlayer},
    packets::server_to_client::{GameServerUpdatePackets, PlayerRemovedFromGame},
};

impl GameServer {
    pub fn leave_game_handler(&mut self, actor_id: u32) -> Result<(), AppError> {
        join_room_handler(self, MAIN_CHAT_ROOM, actor_id)?;
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
        let connected_user = self.sessions.get_mut(&actor_id).ok_or(AppError {
            error_type: common::errors::AppErrorTypes::ServerError,
            message: "Tried to leave game but no user was found".to_string(),
        })?;
        let game_name_leaving = connected_user.current_game_name.clone().ok_or(AppError {
            error_type: common::errors::AppErrorTypes::ServerError,
            message: "User missing reference to their current game".to_string(),
        })?;
        let game = self.games.get_mut(&game_name_leaving).ok_or(AppError {
            error_type: common::errors::AppErrorTypes::ServerError,
            message: "User's current game reference pointed to a game that doesn't exist"
                .to_string(),
        })?;
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
