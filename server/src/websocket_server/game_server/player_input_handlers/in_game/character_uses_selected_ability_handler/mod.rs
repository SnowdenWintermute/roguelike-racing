mod take_next_ai_turn;
use self::take_next_ai_turn::take_ai_controlled_turns;
use super::apply_action_results::apply_action_results;
use crate::websocket_server::game_server::getters::get_mut_game_data_from_actor_id;
use crate::websocket_server::game_server::getters::ActorIdAssociatedGameData;
use crate::websocket_server::game_server::GameServer;
use common::app_consts::error_messages;
use common::combat::ability_handlers::validate_ability_use::validate_character_ability_use;
use common::combat::CombatTurnResult;
use common::errors::AppError;
use common::game::getters::get_mut_party;
use common::packets::server_to_client::CombatTurnResultsPacket;
use common::packets::server_to_client::GameServerUpdatePackets;
use common::packets::WebsocketChannelNamespace;

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
        let party = get_mut_party(game, party_id)?;
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
        println!("{:#?}", character.combatant_properties.ability_targets);
        let targets = character
            .combatant_properties
            .ability_targets
            .clone()
            .ok_or_else(|| AppError {
                error_type: common::errors::AppErrorTypes::InvalidInput,
                message: error_messages::NO_POSSIBLE_TARGETS_PROVIDED.to_string(),
            })?;

        // if in combat
        if let Some(battle_id) = party.battle_id {
            let battle = game.battles.get(&battle_id).ok_or_else(|| AppError {
                error_type: common::errors::AppErrorTypes::ServerError,
                message: error_messages::BATTLE_NOT_FOUND.to_string(),
            })?;

            let (ally_ids, _) = battle.get_ally_ids_and_opponent_ids_option(character_id)?;

            validate_character_ability_use(
                &ability_name,
                &ability_attributes,
                Some(battle),
                &ally_ids,
                &targets,
                character_id,
                game,
            )?;

            let action_results = game.get_ability_results(
                character_id,
                &ability_name,
                &targets,
                Some(&battle.clone()),
            )?;

            apply_action_results(game, &action_results)?;

            let turn_should_end = {
                let mut should_end = false;
                for action_result in &action_results {
                    if action_result.ends_turn {
                        should_end = true;
                        break;
                    }
                }
                should_end
            };

            if turn_should_end {
                let mut turns: Vec<CombatTurnResult> = vec![];
                let player_turn = CombatTurnResult {
                    combatant_id: character_id,
                    action_results,
                };
                turns.push(player_turn);

                let active_combatant_turn_tracker = game.end_active_combatant_turn(battle_id)?;
                let active_combatant_id = active_combatant_turn_tracker.entity_id;

                let mut ai_controlled_turn_results =
                    take_ai_controlled_turns(game, battle_id, active_combatant_id)?;
                // println!("AI turn results: {:#?}", ai_controlled_turn_results);
                turns.append(&mut ai_controlled_turn_results);
                // Send turn result packets
                return self.emit_packet(
                    &party_websocket_channel_name,
                    &WebsocketChannelNamespace::Party,
                    &GameServerUpdatePackets::CombatTurnResults(CombatTurnResultsPacket {
                        turn_results: turns,
                    }),
                    None,
                );
            } else {
                return self.emit_packet(
                    &party_websocket_channel_name,
                    &WebsocketChannelNamespace::Party,
                    &GameServerUpdatePackets::ActionResults(action_results),
                    None,
                );
            }
        } else {
            let game = self.games.get(&current_game_name).ok_or_else(|| AppError {
                error_type: common::errors::AppErrorTypes::ServerError,
                message: error_messages::GAME_NOT_FOUND.to_string(),
            })?;
            // check if ability can be used out of combat
            validate_character_ability_use(
                &ability_name,
                &ability_attributes,
                None,
                &character_positions,
                &targets,
                character_id,
                game,
            )?;

            let game = self
                .games
                .get_mut(&current_game_name)
                .ok_or_else(|| AppError {
                    error_type: common::errors::AppErrorTypes::ServerError,
                    message: error_messages::GAME_NOT_FOUND.to_string(),
                })?;
            let action_results =
                game.get_ability_results(character_id, &ability_name, &targets, None)?;

            apply_action_results(game, &action_results)?;

            // return the targets and hp/mp/status effect changes.
            // client can construct animation of the effects
            return self.emit_packet(
                &party_websocket_channel_name,
                &WebsocketChannelNamespace::Party,
                &GameServerUpdatePackets::ActionResults(action_results),
                None,
            );
        }
    }
}
