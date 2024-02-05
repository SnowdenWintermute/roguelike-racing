use common::app_consts::error_messages;
use common::combat::ActionResult;
use common::combat::CombatTurnResult;
use common::errors::AppError;
use common::packets::server_to_client::CombatTurnResultsPacket;
use common::packets::server_to_client::GameServerUpdatePackets;
use common::packets::WebsocketChannelNamespace;
use crate::websocket_server::game_server::GameServer;
use crate::websocket_server::game_server::player_input_handlers::in_game::character_uses_selected_ability_handler::take_next_ai_turn::take_ai_controlled_turns_if_appropriate;

impl GameServer {
    pub fn handle_end_of_player_character_turn(
        &mut self,
        game_name: &String,
        action_taker_character_id: u32,
        action_results: Vec<ActionResult>,
        all_opponents_are_dead: bool,
        all_allies_are_dead: bool,
        battle_id: u32,
        party_websocket_channel_name: &String,
    ) -> Result<(), AppError> {
        let game = self.games.get_mut(game_name).ok_or_else(|| AppError {
            error_type: common::errors::AppErrorTypes::ServerError,
            message: error_messages::GAME_NOT_FOUND.to_string(),
        })?;
        let mut turns: Vec<CombatTurnResult> = vec![];
        let player_turn = CombatTurnResult {
            combatant_id: action_taker_character_id,
            action_results,
        };
        turns.push(player_turn);

        if !all_opponents_are_dead && !all_allies_are_dead {
            let _ = game.end_active_combatant_turn(battle_id)?;

            let mut ai_controlled_turn_results =
                take_ai_controlled_turns_if_appropriate(game, battle_id)?;
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
        )
    }
}
