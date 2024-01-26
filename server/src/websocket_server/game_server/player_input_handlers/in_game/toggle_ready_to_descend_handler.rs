use std::time::SystemTime;
use std::time::UNIX_EPOCH;

use crate::websocket_server::game_server::getters::get_mut_game;
use crate::websocket_server::game_server::getters::get_mut_user;
use crate::websocket_server::game_server::GameServer;
use common::app_consts::error_messages;
use common::app_consts::LEVEL_TO_REACH_FOR_ESCAPE;
use common::dungeon_rooms::DungeonRoomTypes;
use common::errors::AppError;
use common::game::getters::get_mut_party;
use common::game::getters::get_mut_player;
use common::packets::server_to_client::GameServerUpdatePackets;
use common::packets::GameMessages;
use common::packets::WebsocketChannelNamespace;

impl GameServer {
    pub fn toggle_ready_to_descend_handler(&mut self, actor_id: u32) -> Result<(), AppError> {
        let user = get_mut_user(&mut self.sessions, actor_id)?;
        let username = user.username.clone();
        let game_name = user.current_game_name.clone().ok_or_else(|| AppError {
            error_type: common::errors::AppErrorTypes::ServerError,
            message: error_messages::MISSING_GAME_REFERENCE.to_string(),
        })?;
        let game = get_mut_game(&mut self.games, &game_name)?;
        let player = get_mut_player(game, &username)?;
        let party_id = player.party_id.ok_or_else(|| AppError {
            error_type: common::errors::AppErrorTypes::ServerError,
            message: error_messages::MISSING_PARTY_REFERENCE.to_string(),
        })?;
        let party = get_mut_party(game, party_id)?;
        let party_websocket_channel_name = party.websocket_channel_name.clone();

        if party.current_room.room_type != DungeonRoomTypes::Stairs {
            return Err(AppError {
                error_type: common::errors::AppErrorTypes::InvalidInput,
                message: error_messages::CANT_DESCEND_IF_NO_STAIRS_ARE_PRESENT.to_string(),
            });
        }

        // can't be trying to explore and descend at the same time
        if party.players_ready_to_explore.contains(&username) {
            party.players_ready_to_explore.remove(&username);
        }
        if party.players_ready_to_descend.contains(&username) {
            party.players_ready_to_descend.remove(&username);
        } else {
            party.players_ready_to_descend.insert(username.clone());
        }

        self.emit_packet(
            &party_websocket_channel_name,
            &WebsocketChannelNamespace::Party,
            &GameServerUpdatePackets::PlayerToggledReadyToDescend(username),
            None,
        )?;

        let game = get_mut_game(&mut self.games, &game_name)?;
        let party = get_mut_party(game, party_id)?;
        // if all players names are in the ready to explore list, generate the next room and remove
        // them all from the ready list
        let mut all_players_ready_to_descend = true;
        for username in &party.player_usernames {
            if party.players_ready_to_descend.contains(username) {
                continue;
            } else {
                all_players_ready_to_descend = false;
                break;
            }
        }

        if all_players_ready_to_descend {
            // increase the floor count
            party.current_floor += 1;
            let party_name = party.name.clone();
            let new_party_floor = party.current_floor;
            party.unexplored_rooms.clear();
            let current_floor = party.current_floor;
            party.players_ready_to_descend.clear();
            let player_usernames = party.player_usernames.clone();
            let mut actor_ids = vec![];
            for username in player_usernames {
                let player = game.players.get(&username).ok_or_else(|| AppError {
                    error_type: common::errors::AppErrorTypes::ServerError,
                    message: error_messages::PLAYER_NOT_FOUND.to_string(),
                })?;
                let player_actor_id = player.actor_id.ok_or_else(|| AppError {
                    error_type: common::errors::AppErrorTypes::ServerError,
                    message: error_messages::ACTOR_ID_NOT_FOUND.to_string(),
                })?;
                actor_ids.push(player_actor_id.clone())
            }
            // tell clients their floor number increased
            self.emit_packet(
                &party_websocket_channel_name,
                &WebsocketChannelNamespace::Party,
                &GameServerUpdatePackets::DungeonFloorNumber(current_floor),
                None,
            )?;

            self.emit_packet(
                &game_name,
                &WebsocketChannelNamespace::Game,
                &GameServerUpdatePackets::GameMessage(GameMessages::PartyDescent(
                    party_name.clone(),
                    new_party_floor,
                )),
                None,
            )?;

            if new_party_floor == LEVEL_TO_REACH_FOR_ESCAPE {
                let time_of_escape = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .expect("time went backwards")
                    .as_secs();
                let game = get_mut_game(&mut self.games, &game_name)?;
                let party = get_mut_party(game, party_id)?;
                party.time_of_escape = Some(time_of_escape);
                self.emit_packet(
                    &game_name,
                    &WebsocketChannelNamespace::Game,
                    &GameServerUpdatePackets::GameMessage(GameMessages::PartyEscape(
                        party_name,
                        time_of_escape,
                    )),
                    None,
                )?;
            }

            for player_actor_id in actor_ids {
                self.toggle_ready_to_explore_handler(player_actor_id)?;
            }
            //
            // @TODO - if next room would be stairs and the party is on the final floor, set their
            // time of escape
            // @TODO - if current room is stairs, reset the room order on the current floor
        }

        Ok(())
    }
}
