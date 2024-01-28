mod take_next_ai_turn;
use self::take_next_ai_turn::take_ai_controlled_turns;
use super::apply_action_results::apply_action_results;
use crate::websocket_server::game_server::getters::get_mut_game_data_from_actor_id;
use crate::websocket_server::game_server::getters::ActorIdAssociatedGameData;
use crate::websocket_server::game_server::GameServer;
use common::app_consts::error_messages;
use common::combat::ability_handlers::validate_ability_use::validate_character_ability_use;
use common::combat::CombatTurnResult;
use common::dungeon_rooms::DungeonRoomTypes;
use common::errors::AppError;
use common::game::getters::get_mut_party;
use common::items::Item;
use common::items::ItemCategories;
use common::packets::server_to_client::BattleConclusion;
use common::packets::server_to_client::BattleEndReportPacket;
use common::packets::server_to_client::CombatTurnResultsPacket;
use common::packets::server_to_client::GameServerUpdatePackets;
use common::packets::WebsocketChannelNamespace;
use std::time::SystemTime;
use std::time::UNIX_EPOCH;

impl GameServer {
    pub fn character_uses_selected_ability_handler(
        &mut self,
        actor_id: u32,
        character_id: u32,
    ) -> Result<(), AppError> {
        let ActorIdAssociatedGameData {
            game,
            party_id,
            player_character_ids_option,
            current_game_name,
            ..
        } = get_mut_game_data_from_actor_id(self, actor_id)?;
        let party_id = party_id.clone();
        let current_game_name = current_game_name.clone();
        let party = get_mut_party(game, party_id)?;
        let party_name = party.name.clone();
        let in_monster_lair = { party.current_room.room_type == DungeonRoomTypes::MonsterLair };
        let dlvl = party.current_floor;
        let battle_id_option = party.battle_id;
        let character_positions = party.character_positions.clone();
        let party_websocket_channel_name = party.websocket_channel_name.clone();
        let character =
            party.get_mut_character_if_owned(player_character_ids_option, character_id)?;
        let ability_name = character
            .combatant_properties
            .selected_ability_name
            .clone()
            .ok_or_else(|| AppError {
                error_type: common::errors::AppErrorTypes::InvalidInput,
                message: error_messages::NO_ABILITY_SELECTED.to_string(),
            })?;
        let ability_attributes = ability_name.get_attributes();
        // check if they own the ability
        let _ = character
            .combatant_properties
            .get_ability_if_owned(&ability_name)?;

        let targets = character
            .combatant_properties
            .ability_targets
            .clone()
            .ok_or_else(|| AppError {
                error_type: common::errors::AppErrorTypes::InvalidInput,
                message: error_messages::NO_POSSIBLE_TARGETS_PROVIDED.to_string(),
            })?;

        let battle_option = if let Some(battle_id) = battle_id_option {
            Some(
                game.battles
                    .get(&battle_id)
                    .ok_or_else(|| AppError {
                        error_type: common::errors::AppErrorTypes::ServerError,
                        message: error_messages::BATTLE_NOT_FOUND.to_string(),
                    })?
                    .clone(),
            )
        } else {
            None
        };

        let ally_ids = if let Some(battle) = &battle_option {
            let (ally_ids, _) = battle.get_ally_ids_and_opponent_ids_option(character_id)?;
            ally_ids
        } else {
            character_positions.clone()
        };

        validate_character_ability_use(
            &ability_attributes.combat_action_properties,
            battle_option.as_ref(),
            &ally_ids,
            &targets,
            character_id,
        )?;

        let action_results = game.get_ability_results(
            character_id,
            &ability_name,
            &targets,
            battle_option.as_ref(),
        )?;

        apply_action_results(game, &action_results)?;

        // check if all enemies/allies are dead
        let all_opponents_are_dead = if let Some(battle) = &battle_option {
            let (_, opponent_ids_option) =
                battle.get_ally_ids_and_opponent_ids_option(character_id)?;
            if let Some(opponent_ids) = opponent_ids_option {
                game.all_combatants_in_group_are_dead(opponent_ids)?
            } else {
                false
            }
        } else {
            false
        };

        let all_allies_are_dead = game.all_combatants_in_group_are_dead(ally_ids.clone())?;

        let num_opponents = if let Some(battle) = &battle_option {
            let (_, opponent_ids_option) =
                battle.get_ally_ids_and_opponent_ids_option(character_id)?;
            opponent_ids_option.unwrap_or_else(|| vec![]).len()
        } else {
            0
        };

        let game = self
            .games
            .get_mut(&current_game_name)
            .ok_or_else(|| AppError {
                error_type: common::errors::AppErrorTypes::ServerError,
                message: error_messages::GAME_NOT_FOUND.to_string(),
            })?;
        let loot_option = if in_monster_lair && all_opponents_are_dead {
            let mut loot = vec![];
            println!("creating loot for dlvl {dlvl}");
            for _ in 0..num_opponents {
                // for _ in 0..30 {
                loot.push(Item::generate(
                    &mut game.id_generator,
                    dlvl,
                    ItemCategories::Consumable,
                ))
            }
            Some(loot)
        } else {
            None
        };
        let party = get_mut_party(game, party_id)?;
        if all_opponents_are_dead {
            println!("all opponents defeated, concluding battle as victory");
            // delete the battle
            party.battle_id = None;
            if in_monster_lair {
                party.current_room.monsters = None;
                party.current_room.items = loot_option.clone().unwrap_or_default();
            }
            if let Some(battle_id) = battle_id_option {
                game.battles.remove(&battle_id);
            }
        }

        // if in combat
        let used_turn_ending_ability_in_battle = if battle_id_option.is_some() {
            let mut should_end = false;
            for action_result in &action_results {
                if action_result.ends_turn {
                    should_end = true;
                    break;
                }
            }
            should_end
        } else {
            false
        };

        if used_turn_ending_ability_in_battle {
            let mut turns: Vec<CombatTurnResult> = vec![];
            let player_turn = CombatTurnResult {
                combatant_id: character_id,
                action_results,
            };
            turns.push(player_turn);

            if !all_opponents_are_dead && !all_allies_are_dead {
                let active_combatant_turn_tracker =
                    game.end_active_combatant_turn(battle_id_option.expect("checked above"))?;
                let active_combatant_id = active_combatant_turn_tracker.entity_id;

                let mut ai_controlled_turn_results = take_ai_controlled_turns(
                    game,
                    battle_id_option.expect("checked above"),
                    active_combatant_id,
                )?;
                turns.append(&mut ai_controlled_turn_results);
            }

            // Send turn result packets
            self.emit_packet(
                &party_websocket_channel_name,
                &WebsocketChannelNamespace::Party,
                &GameServerUpdatePackets::CombatTurnResults(CombatTurnResultsPacket {
                    turn_results: turns,
                }),
                None,
            )?;
        } else {
            self.emit_packet(
                &party_websocket_channel_name,
                &WebsocketChannelNamespace::Party,
                &GameServerUpdatePackets::ActionResults(action_results),
                None,
            )?;
        }

        let game = self
            .games
            .get_mut(&current_game_name)
            .ok_or_else(|| AppError {
                error_type: common::errors::AppErrorTypes::ServerError,
                message: error_messages::GAME_NOT_FOUND.to_string(),
            })?;
        let all_players_in_game = game.players.clone();
        let all_allies_are_dead = game.all_combatants_in_group_are_dead(ally_ids)?;
        let party = get_mut_party(game, party_id)?;
        let player_usernames_in_party = party.player_usernames.clone();
        if all_allies_are_dead {
            println!(
                "all allies are dead, ending game for party named '{}'",
                party.name
            );
            party.battle_id = None;
            if let Some(battle_id) = battle_id_option {
                game.battles.remove(&battle_id);
            }
            let mut actor_ids_of_players_in_other_parties = Vec::new();
            for (username, player) in &all_players_in_game {
                if !player_usernames_in_party.contains(username) {
                    actor_ids_of_players_in_other_parties.push(player.actor_id)
                }
            }
            for actor_id_option in actor_ids_of_players_in_other_parties {
                if let Some(actor_id) = actor_id_option {
                    self.send_packet(
                        &GameServerUpdatePackets::GameMessage(
                            common::packets::GameMessages::PartyWipe(
                                party_name.clone(),
                                dlvl,
                                SystemTime::now()
                                    .duration_since(UNIX_EPOCH)
                                    .expect("time went backwards")
                                    .as_secs(),
                            ),
                        ),
                        actor_id,
                    )?;
                }
            }

            self.emit_packet(
                &party_websocket_channel_name,
                &WebsocketChannelNamespace::Party,
                &GameServerUpdatePackets::BattleEndReport(BattleEndReportPacket {
                    conclusion: BattleConclusion::Defeat,
                    loot: None,
                }),
                None,
            )?;

            let game = self
                .games
                .get_mut(&current_game_name)
                .ok_or_else(|| AppError {
                    error_type: common::errors::AppErrorTypes::ServerError,
                    message: error_messages::GAME_NOT_FOUND.to_string(),
                })?;
            for username in player_usernames_in_party {
                println!("attempting to remove username {} from party", username);
                game.remove_player_from_adventuring_party(username)?;
            }

            if game.adventuring_parties.len() == 0 {
                for (_, player) in all_players_in_game {
                    if let Some(actor_id) = player.actor_id {
                        println!("attempting to remove actor id {} from game", actor_id);
                        let result = self.remove_player_from_game(actor_id);
                        if let Some(err) = result.err() {
                            println!("error removing player from game {}", err.message)
                        }
                    }
                }
            }
        } else if in_monster_lair && all_opponents_are_dead {
            if let Some(loot) = loot_option.clone() {
                // make sure all clients receive the item's existance or else one client can take
                // the item before another client sees it leading to desync
                let game = self
                    .games
                    .get_mut(&current_game_name)
                    .ok_or_else(|| AppError {
                        error_type: common::errors::AppErrorTypes::ServerError,
                        message: error_messages::GAME_NOT_FOUND.to_string(),
                    })?;
                let party = get_mut_party(game, party_id)?;
                for item in loot {
                    party
                        .items_on_ground_not_yet_received_by_all_clients
                        .insert(item.entity_properties.id, vec![]);
                }
            };
            self.emit_packet(
                &party_websocket_channel_name,
                &WebsocketChannelNamespace::Party,
                &GameServerUpdatePackets::BattleEndReport(BattleEndReportPacket {
                    conclusion: BattleConclusion::Victory,
                    loot: loot_option,
                }),
                None,
            )?;
        }

        Ok(())
    }
}
