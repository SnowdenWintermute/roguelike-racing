mod action_result_ended_turn;
mod generate_loot_if_appropriate;
mod get_character_ally_ids;
mod get_number_of_opponents;
mod handle_end_of_player_character_turn;
mod handle_party_battle_victory;
mod handle_party_wipe;
use self::action_result_ended_turn::action_result_ended_turn;
use self::get_character_ally_ids::get_character_ally_ids;
use self::get_number_of_opponents::get_number_of_opponents;
use super::apply_action_results::apply_action_results;
use crate::websocket_server::game_server::getters::get_mut_game_data_from_actor_id;
use crate::websocket_server::game_server::getters::ActorIdAssociatedGameData;
use crate::websocket_server::game_server::GameServer;
use common::combat::ActionResult;
use common::dungeon_rooms::DungeonRoomTypes;
use common::errors::AppError;
use common::game::getters::get_party;
use common::packets::server_to_client::ActionResultsPacket;
use common::packets::server_to_client::GameServerUpdatePackets;
use common::packets::WebsocketChannelNamespace;

impl GameServer {
    pub fn handle_new_combat_action_results(
        &mut self,
        actor_id: u32,
        action_results: Vec<ActionResult>,
        action_taker_id: u32,
    ) -> Result<(), AppError> {
        let ActorIdAssociatedGameData {
            game,
            party_id,
            current_game_name,
            ..
        } = get_mut_game_data_from_actor_id(self, actor_id)?;

        apply_action_results(game, &action_results)?;

        let party_id = party_id.clone();
        let current_game_name = current_game_name.clone();
        let party = get_party(game, party_id)?;
        let in_monster_lair = { party.current_room.room_type == DungeonRoomTypes::MonsterLair };
        let battle_id_option = party.battle_id;
        let character_positions = party.character_positions.clone();
        let party_websocket_channel_name = party.websocket_channel_name.clone();
        let battle_option = game.get_battle_option(&battle_id_option)?;
        let ally_ids =
            get_character_ally_ids(&battle_option, &character_positions, &action_taker_id)?;
        let ActorIdAssociatedGameData { game, .. } =
            get_mut_game_data_from_actor_id(self, actor_id)?;

        // check if all enemies/allies are dead
        let all_opponents_are_dead = if let Some(battle) = &battle_option {
            let (_, opponent_ids_option) =
                battle.get_ally_ids_and_opponent_ids_option(action_taker_id)?;
            if let Some(opponent_ids) = opponent_ids_option {
                game.all_combatants_in_group_are_dead(opponent_ids)?
            } else {
                false
            }
        } else {
            false
        };

        let all_allies_are_dead = game.all_combatants_in_group_are_dead(ally_ids.clone())?;
        let num_opponents = get_number_of_opponents(&battle_option, action_taker_id)?;

        // let party = get_mut_party(game, party_id)?;

        let action_result_ended_turn = action_result_ended_turn(battle_id_option, &action_results);
        if action_result_ended_turn {
            self.handle_end_of_player_character_turn(
                &current_game_name,
                action_taker_id,
                action_results,
                all_opponents_are_dead,
                all_allies_are_dead,
                battle_id_option.expect("checked in action_result_ended_turn fn"),
                &party_websocket_channel_name,
            )?;
        } else {
            self.emit_packet(
                &party_websocket_channel_name,
                &WebsocketChannelNamespace::Party,
                &GameServerUpdatePackets::ActionResults(ActionResultsPacket {
                    action_taker_id,
                    action_results,
                }),
                None,
            )?;
        }

        let ActorIdAssociatedGameData { game, .. } =
            get_mut_game_data_from_actor_id(self, actor_id)?;
        let all_allies_are_dead = game.all_combatants_in_group_are_dead(ally_ids)?;

        if all_allies_are_dead {
            self.handle_party_wipe(actor_id, &party_websocket_channel_name, &battle_id_option)?;
        } else if in_monster_lair && all_opponents_are_dead {
            self.handle_party_battle_victory(
                actor_id,
                battle_id_option.expect("to be in a battle"),
                num_opponents,
            )?;
        }

        Ok(())
    }
}
