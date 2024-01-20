use crate::websocket_server::game_server::getters::get_mut_game_data_from_actor_id;
use crate::websocket_server::game_server::getters::ActorIdAssociatedGameData;
use crate::websocket_server::game_server::GameServer;
use common::app_consts::error_messages;
use common::errors::AppError;
use common::game::getters::get_mut_party;
use common::packets::server_to_client::GameServerUpdatePackets;
use common::packets::CharacterAndSlot;
use common::packets::WebsocketChannelNamespace;

impl GameServer {
    pub fn character_drops_equipped_item(
        &mut self,
        actor_id: u32,
        packet: CharacterAndSlot,
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
        let character = party
            .get_mut_character_if_owned(player_character_ids_option.clone(), packet.character_id)?;
        let character_id = character.entity_properties.id;

        let item = character
            .combatant_properties
            .equipment
            .remove(&packet.slot)
            .ok_or_else(|| AppError {
                error_type: common::errors::AppErrorTypes::InvalidInput,
                message: error_messages::TRIED_TO_DROP_ITEM_FROM_AN_EMPTY_SLOT.to_string(),
            })?;
        let item_id = item.entity_properties.id;
        party.current_room.items.push(item);

        party
            .items_on_ground_not_yet_received_by_all_clients
            .insert(item_id, Vec::new());

        self.emit_packet(
            &party_websocket_channel_name,
            &WebsocketChannelNamespace::Party,
            &GameServerUpdatePackets::CharacterDroppedEquippedItem(CharacterAndSlot {
                character_id,
                slot: packet.slot,
            }),
            None,
        )
    }
}
