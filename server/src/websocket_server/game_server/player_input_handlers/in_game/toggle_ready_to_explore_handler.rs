use std::collections::HashSet;

use crate::websocket_server::game_server::{
    getters::{get_mut_game, get_mut_user},
    GameServer,
};
use common::{
    app_consts::error_messages,
    dungeon_rooms::{DungeonRoom, DungeonRoomTypes},
    errors::AppError,
    game::getters::{get_mut_party, get_mut_player},
    packets::server_to_client::GameServerUpdatePackets,
};

impl GameServer {
    pub fn toggle_ready_to_explore_handler(&mut self, actor_id: u32) -> Result<(), AppError> {
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

        if party.current_room.monsters.is_some() {
            return Err(AppError {
                error_type: common::errors::AppErrorTypes::InvalidInput,
                message: error_messages::CANT_EXPLORE_WHEN_MONSTERS_ARE_PRESENT.to_string(),
            });
        }

        let current_floor = party.current_floor;
        //
        if party.players_ready_to_explore.contains(&username) {
            party.players_ready_to_explore.remove(&username);
        } else {
            party.players_ready_to_explore.insert(username.clone());
        };

        // if all players names are in the ready to explore list, generate the next room and remove
        // them all from the ready list
        let mut all_players_ready_to_explore = true;
        for username in &party.player_usernames {
            if party.players_ready_to_explore.contains(username) {
                continue;
            } else {
                all_players_ready_to_explore = false;
                break;
            }
        }
        println!(
            "all players ready to explore: {}",
            all_players_ready_to_explore
        );

        let mut new_room = None;
        if all_players_ready_to_explore {
            party.players_ready_to_explore = HashSet::new();
            new_room = Some(DungeonRoom::generate(
                &mut game.id_generator,
                current_floor,
                false,
                Some(DungeonRoomTypes::MonsterLair),
            ));
        }

        let party = get_mut_party(game, party_id)?;
        if let Some(room) = new_room {
            party.current_room = room.clone();
            party.rooms_explored.on_current_floor += 1;
            party.rooms_explored.total += 1;

            if room.monsters.is_some() {
                let turn_trackers = party.get_combat_turn_order();
                party.combatant_turn_trackers = Some(turn_trackers);
            }

            self.emit_packet(
                &game_name,
                &GameServerUpdatePackets::DungeonRoomUpdate(room),
                None,
            )?;
        }

        self.emit_packet(
            &game_name,
            &GameServerUpdatePackets::PlayerToggledReadyToExplore(username),
            None,
        )?;
        Ok(())
    }
}
