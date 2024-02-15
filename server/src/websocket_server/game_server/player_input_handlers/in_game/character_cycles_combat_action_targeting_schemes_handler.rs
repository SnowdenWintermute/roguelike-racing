use crate::websocket_server::game_server::getters::get_mut_game_data_from_actor_id;
use crate::websocket_server::game_server::getters::ActorIdAssociatedGameData;
use crate::websocket_server::game_server::GameServer;
use common::errors::AppError;
use common::game::getters::get_party;
use common::packets::server_to_client::GameServerUpdatePackets;
use common::packets::CharacterId;
use common::packets::WebsocketChannelNamespace;

impl GameServer {
    pub fn character_cycles_combat_action_targeting_schemes_handler(
        &mut self,
        actor_id: u32,
        character_id: CharacterId,
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

        game.cycle_targeting_schemes(
            party_id,
            player_character_ids_option,
            &username,
            character_id,
        )?;

        self.emit_packet(
            &party_websocket_channel_name,
            &WebsocketChannelNamespace::Party,
            &GameServerUpdatePackets::CharacterCycledCombatActionTargetingSchemes(character_id),
            Some(actor_id),
        )
    }
}
