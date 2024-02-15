use crate::websocket_server::game_server::getters::get_mut_game_data_from_actor_id;
use crate::websocket_server::game_server::getters::ActorIdAssociatedGameData;
use crate::websocket_server::game_server::GameServer;
use common::errors::AppError;
use common::game::getters::get_party;
use common::packets::server_to_client::GameServerUpdatePackets;
use common::packets::CharacterAndDirection;
use common::packets::WebsocketChannelNamespace;

impl GameServer {
    pub fn character_cycles_combat_action_targets_handler(
        &mut self,
        actor_id: u32,
        packet: CharacterAndDirection,
    ) -> Result<(), AppError> {
        let ActorIdAssociatedGameData {
            game,
            party_id,
            player_character_ids_option,
            username,
            ..
        } = get_mut_game_data_from_actor_id(self, actor_id)?;
        let party = get_party(game, party_id)?;
        let party_websocket_channel_name = party.websocket_channel_name.clone();

        game.cycle_character_targets(
            party_id,
            player_character_ids_option,
            &username,
            packet.character_id,
            &packet.direction,
        )?;

        self.emit_packet(
            &party_websocket_channel_name,
            &WebsocketChannelNamespace::Party,
            &GameServerUpdatePackets::CharacterCycledCombatActionTargets(packet),
            Some(actor_id),
        )
    }
}
