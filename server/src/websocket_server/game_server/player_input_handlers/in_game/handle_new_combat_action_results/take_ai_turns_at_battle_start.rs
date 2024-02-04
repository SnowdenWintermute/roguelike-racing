use common::app_consts::error_messages;
use common::combat::CombatTurnResult;
use common::errors::AppError;
use common::errors::AppErrorTypes;
use common::packets::server_to_client::CombatTurnResultsPacket;
use common::packets::server_to_client::GameServerUpdatePackets;
use common::packets::WebsocketChannelNamespace;
use crate::websocket_server::game_server::GameServer;
use crate::websocket_server::game_server::player_input_handlers::in_game::character_uses_selected_ability_handler::take_next_ai_turn::take_ai_controlled_turns_if_appropriate;

impl GameServer {
    pub fn take_ai_turns_at_battle_start(
        &mut self,
        game_name: &String,
        battle_id: u32,
        party_websocket_channel_name: &String,
    ) -> Result<(), AppError> {
        let mut turns: Vec<CombatTurnResult> = vec![];

        let game = self.games.get_mut(game_name).ok_or_else(|| AppError {
            error_type: AppErrorTypes::ServerError,
            message: error_messages::GAME_NOT_FOUND.to_string(),
        })?;

        let mut ai_controlled_turn_results =
            take_ai_controlled_turns_if_appropriate(game, battle_id)?;
        turns.append(&mut ai_controlled_turn_results);
        // Send turn result packets
        if turns.len() > 0 {
            self.emit_packet(
                &party_websocket_channel_name,
                &WebsocketChannelNamespace::Party,
                &GameServerUpdatePackets::CombatTurnResults(CombatTurnResultsPacket {
                    turn_results: turns,
                }),
                None,
            )?;
        }
        Ok(())
    }
}
