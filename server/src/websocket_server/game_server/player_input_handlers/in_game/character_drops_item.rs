use crate::websocket_server::game_server::getters::get_mut_game_data_from_actor_id;
use crate::websocket_server::game_server::getters::ActorIdAssociatedGameData;
use crate::websocket_server::game_server::GameServer;
use common::errors::AppError;
use common::game::getters::get_mut_party;
use common::packets::server_to_client::GameServerUpdatePackets;
use common::packets::CharacterAndItem;
use common::packets::WebsocketChannelNamespace;

impl GameServer {
    pub fn character_drops_item(
        &mut self,
        actor_id: u32,
        packet: CharacterAndItem,
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

        let item = character.combatant_properties.inventory.remove_item(packet.item_id)?;
        let item_id = item.entity_properties.id;
        party.current_room.items.push(item);

        party
            .items_on_ground_not_yet_received_by_all_clients
            .insert(item_id, Vec::new());

        self.emit_packet(
            &party_websocket_channel_name,
            &WebsocketChannelNamespace::Party,
            &GameServerUpdatePackets::CharacterDroppedItem(CharacterAndItem {
                character_id,
                item_id,
            }),
            None,
        )
    }
}
