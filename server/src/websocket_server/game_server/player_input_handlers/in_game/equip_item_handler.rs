use crate::websocket_server::game_server::getters::get_mut_party_game_name_and_character_ids_from_actor_id;
use crate::websocket_server::game_server::getters::ActorIdAssociatedPartyData;
use crate::websocket_server::game_server::GameServer;
use common::app_consts::error_messages;
use common::errors::AppError;
use common::packets::server_to_client::CharacterEquippedItemPacket;
use common::packets::server_to_client::GameServerUpdatePackets;
use common::packets::WebsocketChannelNamespace;

impl GameServer {
    pub fn equip_item_handler(
        &mut self,
        actor_id: u32,
        character_id: u32,
        item_id: u32,
        alt_slot: bool,
    ) -> Result<(), AppError> {
        let ActorIdAssociatedPartyData {
            party,
            current_game_name,
            player_character_ids_option,
            ..
        } = get_mut_party_game_name_and_character_ids_from_actor_id(self, actor_id)?;

        let player_character_ids = player_character_ids_option.ok_or_else(|| AppError {
            error_type: common::errors::AppErrorTypes::ServerError,
            message: error_messages::PLAYER_HAS_NO_CHARACTERS.to_string(),
        })?;

        let character = match player_character_ids.contains(&character_id) {
            true => party
                .characters
                .get_mut(&character_id)
                .ok_or_else(|| AppError {
                    error_type: common::errors::AppErrorTypes::ServerError,
                    message: error_messages::CHARACTER_NOT_FOUND.to_string(),
                }),
            false => Err(AppError {
                error_type: common::errors::AppErrorTypes::ServerError,
                message: error_messages::CHARACTER_NOT_OWNED.to_string(),
            }),
        }?;
        character.equip_item(item_id, alt_slot)?;

        self.emit_packet(
            &party.websocket_channel_name,
            &WebsocketChannelNamespace::Party,
            &GameServerUpdatePackets::CharacterEquippedItem(CharacterEquippedItemPacket {
                character_id,
                item_id,
                alt_slot,
            }),
            None,
        )
    }
}
