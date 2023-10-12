use super::{get_user, GameServer};
use common::errors::AppError;
use common::game::RoguelikeRacerGame;
use common::packets::server_to_client::{
    ClientGameListState, GameListEntry, GameServerUpdatePackets, RoguelikeRacerAppState, RoomState,
};
use serde::{Deserialize, Serialize};
use std::time::Instant;

impl GameServer {
    pub fn create_game_list_update(&self) -> ClientGameListState {
        let mut game_list = ClientGameListState { games: Vec::new() };
        for (game_name, game) in self.games.iter() {
            game_list.games.push(GameListEntry {
                game_name: game_name.to_string(),
                number_of_users: game.get_number_of_players(),
                time_started: game.time_started,
            })
        }
        game_list
    }

    pub fn create_game_full_update(
        &self,
        actor_id: u32,
    ) -> Result<Option<RoguelikeRacerGame>, AppError> {
        let connected_user = get_user(&self.sessions, actor_id)?;
        let current_game_name = connected_user.current_game_name.clone();
        let current_game_option = match current_game_name {
            Some(game_name) => {
                let game = self
                    .games
                    .get(&game_name.to_string())
                    .clone()
                    .ok_or(AppError {
                        error_type: common::errors::AppErrorTypes::ServerError,
                        message:
                            "User's current game reference pointed to a game that doesn't exist"
                                .to_string(),
                    })?;
                Some(game)
            }
            None => None,
        };

        let mut current_game = match current_game_option {
            Some(game) => Some(game.clone()),
            None => None,
        };

        // sanitize actor ids from players
        match current_game {
            Some(ref mut game) => {
                for (_, player) in game.partyless_players.iter_mut() {
                    player.actor_id = None;
                }
                for (_, party) in game.adventuring_parties.iter_mut() {
                    for (_, player) in party.players.iter_mut() {
                        player.actor_id = None;
                    }
                }
            }
            None => (),
        }

        Ok(current_game)
    }

    pub fn create_client_update_packet(
        &mut self,
        actor_id: u32,
    ) -> Result<Option<GameServerUpdatePackets>, AppError> {
        let connected_user = get_user(&self.sessions, actor_id)?;
        let current_game = self.create_game_full_update(actor_id)?;

        let room = self
            .rooms
            .get(&connected_user.current_room_name)
            .ok_or(AppError {
                error_type: common::errors::AppErrorTypes::ServerError,
                message: "if a room is registered with a connected_user then it should exist"
                    .to_string(),
            })?;
        let mut room_update = RoomState {
            room_name: connected_user.current_room_name.clone(),
            users: Vec::new(),
        };

        for actor_id in room.iter() {
            let user = get_user(&mut self.sessions, *actor_id)?;
            room_update.users.push(user.username.clone());
        }

        // GAME LIST UPDATE
        let game_list = self.create_game_list_update();

        let full_update = RoguelikeRacerAppState {
            room: room_update,
            game_list,
            current_game,
        };

        Ok(Some(GameServerUpdatePackets::FullUpdate(full_update)))
    }
}
