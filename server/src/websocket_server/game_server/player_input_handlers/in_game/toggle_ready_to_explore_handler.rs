use crate::websocket_server::game_server::getters::get_mut_game;
use crate::websocket_server::game_server::getters::get_mut_user;
use crate::websocket_server::game_server::GameServer;
use common::app_consts::error_messages;
use common::combat::battle::BattleGroup;
use common::combat::battle::BattleGroupTypes;
use common::dungeon_rooms::DungeonRoom;
use common::dungeon_rooms::DungeonRoomTypes;
use common::errors::AppError;
use common::game::getters::get_mut_party;
use common::game::getters::get_mut_player;
use common::packets::server_to_client::GameServerUpdatePackets;
use common::packets::WebsocketChannelNamespace;
use common::utils::server_log;
use std::collections::VecDeque;

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
        let party_websocket_channel_name = party.websocket_channel_name.clone();

        if party.current_room.monsters.is_some() {
            return Err(AppError {
                error_type: common::errors::AppErrorTypes::InvalidInput,
                message: error_messages::CANT_EXPLORE_WHEN_MONSTERS_ARE_PRESENT.to_string(),
            });
        }

        let current_floor = party.current_floor;
        // can't be trying to explore and descend at the same time
        if party.players_ready_to_descend.contains(&username) {
            party.players_ready_to_descend.remove(&username);
        }
        if party.players_ready_to_explore.contains(&username) {
            party.players_ready_to_explore.remove(&username);
        } else {
            party.players_ready_to_explore.insert(username.clone());
        };

        self.emit_packet(
            &party_websocket_channel_name,
            &WebsocketChannelNamespace::Party,
            &GameServerUpdatePackets::PlayerToggledReadyToExplore(username),
            None,
        )?;

        let game = get_mut_game(&mut self.games, &game_name)?;
        let party = get_mut_party(game, party_id)?;
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

        let mut new_room = None;
        if all_players_ready_to_explore {
            // @TODO - if next room would be stairs and the party is on the final floor, set their
            // time of escape
            // @TODO - if current room is stairs, reset the room order on the current floor

            party.players_ready_to_explore.clear();
            let room_type_to_generate_option = party.unexplored_rooms.pop_front();
            let mut new_room_types_list_for_client_option = None;
            let room_type_to_generate = match room_type_to_generate_option {
                Some(room_type_to_generate) => room_type_to_generate,
                None => {
                    party.generate_unexplored_rooms_queue();
                    new_room_types_list_for_client_option = Some(party.unexplored_rooms.clone());
                    party.unexplored_rooms.pop_front().ok_or_else(|| AppError {
                        error_type: common::errors::AppErrorTypes::ServerError,
                        message: error_messages::MISSING_ROOM_TYPE_TO_GENERATE.to_string(),
                    })?
                }
            };

            new_room = Some(DungeonRoom::generate(
                &mut game.id_generator,
                current_floor,
                false,
                Some(room_type_to_generate),
            ));
            //
            if let Some(new_room_types_list_for_client) = new_room_types_list_for_client_option {
                let new_room_types_list_for_client = new_room_types_list_for_client
                    .into_iter()
                    .map(|item| {
                        if item == DungeonRoomTypes::MonsterLair {
                            Some(DungeonRoomTypes::MonsterLair)
                        } else {
                            None
                        }
                    })
                    .collect::<VecDeque<Option<DungeonRoomTypes>>>();
                self.emit_packet(
                    &party_websocket_channel_name,
                    &WebsocketChannelNamespace::Party,
                    &GameServerUpdatePackets::DungeonRoomTypesOnCurrentFloor(
                        new_room_types_list_for_client,
                    ),
                    None,
                )?;
            }
        }

        let game = get_mut_game(&mut self.games, &game_name)?;
        let party = get_mut_party(game, party_id)?;
        if let Some(room) = new_room {
            party.current_room = room.clone();
            party.rooms_explored.on_current_floor += 1;
            party.rooms_explored.total += 1;

            self.emit_packet(
                &party_websocket_channel_name,
                &WebsocketChannelNamespace::Party,
                &GameServerUpdatePackets::DungeonRoomUpdate(room.clone()),
                None,
            )?;

            let game = get_mut_game(&mut self.games, &game_name)?;
            let party = get_mut_party(game, party_id)?;

            if room.monsters.is_some() {
                let group_a = BattleGroup {
                    name: format!("{}", party.name).to_string(),
                    party_id: party.id,
                    combatant_ids: party.character_positions.clone(),
                    group_type: BattleGroupTypes::PlayerControlled,
                };
                let mut monster_ids = party.get_monster_ids()?;
                monster_ids.sort();
                let group_b = BattleGroup {
                    name: format!("{}-monsters", party.name).to_string(),
                    party_id,
                    combatant_ids: monster_ids,
                    group_type: BattleGroupTypes::ComputerControlled,
                };
                let battle_id = game.initiate_battle(group_a, group_b)?;
                server_log("ticking combat at battle start");

                game.tick_combat_until_next_combatant_is_active(battle_id)?;
                let party = get_mut_party(game, party_id)?;
                party.battle_id = Some(battle_id);
                let battle = game.battles.get(&battle_id).clone().expect("just inserted");

                let cloned_battle = battle.clone();
                self.emit_packet(
                    &party_websocket_channel_name,
                    &WebsocketChannelNamespace::Party,
                    &GameServerUpdatePackets::BattleFullUpdate(Some(cloned_battle)),
                    None,
                )?;
            }

            let game = get_mut_game(&mut self.games, &game_name)?;
            let party = get_mut_party(game, party_id)?;
            let battle_id_option = party.battle_id;
            if let Some(battle_id) = party.battle_id {
                self.take_ai_turns_at_battle_start(
                    &game_name,
                    battle_id,
                    &party_websocket_channel_name,
                )?;
                let game = get_mut_game(&mut self.games, &game_name)?;
                let party = get_mut_party(game, party_id)?;
                let character_positions = party.character_positions.clone();
                let all_allies_are_dead =
                    game.all_combatants_in_group_are_dead(character_positions)?;
                if all_allies_are_dead {
                    self.handle_party_wipe(
                        actor_id,
                        &party_websocket_channel_name,
                        &battle_id_option,
                    )?;
                }
            }
        }
        Ok(())
    }
}
