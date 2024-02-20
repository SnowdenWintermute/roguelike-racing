use crate::websocket_server::game_server::getters::get_mut_game_data_from_actor_id;
use crate::websocket_server::game_server::getters::ActorIdAssociatedGameData;
use crate::websocket_server::game_server::GameServer;
use common::app_consts::error_messages;
use common::combatants::combat_attributes::CombatAttributes;
use common::errors::AppError;
use common::game::getters::get_mut_party;
use common::packets::server_to_client::GameServerUpdatePackets;
use common::packets::WebsocketChannelNamespace;

impl GameServer {
    pub fn character_spends_attribute_point_handler(
        &mut self,
        actor_id: u32,
        character_id: u32,
        attribute: CombatAttributes,
    ) -> Result<(), AppError> {
        let ActorIdAssociatedGameData {
            game,
            party_id,
            player_character_ids_option,
            ..
        } = get_mut_game_data_from_actor_id(self, actor_id)?;
        let party_id = party_id.clone();
        let party = get_mut_party(game, party_id)?;
        let party_websocket_channel_name = party.websocket_channel_name.clone();
        let character =
            party.get_mut_character_if_owned(player_character_ids_option.clone(), character_id)?;

        let combatant_properties = &mut character.combatant_properties;
        if combatant_properties.unspent_attribute_points <= 0 {
            return Err(AppError {
                error_type: common::errors::AppErrorTypes::InvalidInput,
                message: error_messages::NO_UNSPENT_ATTRIBUTE_POINTS.to_string(),
            });
        }

        combatant_properties.unspent_attribute_points -= 1;
        let current_attribute_value = combatant_properties
            .inherent_attributes
            .entry(attribute)
            .or_insert(0);
        *current_attribute_value += 1;

        self.emit_packet(
            &party_websocket_channel_name,
            &WebsocketChannelNamespace::Party,
            &GameServerUpdatePackets::CharacterSpentAttributePoint(character_id, attribute),
            None,
        )
    }
}
