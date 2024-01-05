use crate::websocket_server::game_server::getters::get_mut_party_game_name_and_character_ids_from_actor_id;
use crate::websocket_server::game_server::getters::ActorIdAssociatedPartyData;
use crate::websocket_server::game_server::GameServer;
use common::app_consts::error_messages;
use common::errors::AppError;
use common::items::equipment::EquipmentSlots;
use common::packets::client_to_server::UnequipSlotRequest;
use common::packets::server_to_client::GameServerUpdatePackets;
use common::packets::WebsocketChannelNamespace;

impl GameServer {
    pub fn unequip_slot_handler(
        &mut self,
        actor_id: u32,
        character_id: u32,
        slot: EquipmentSlots,
    ) -> Result<(), AppError> {
        let ActorIdAssociatedPartyData {
            party,
            current_game_name,
            player_character_ids_option,
            ..
        } = get_mut_party_game_name_and_character_ids_from_actor_id(self, actor_id)?;
        let party_websocket_channel_name = party.websocket_channel_name.clone();

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

        character.unequip_slots(&vec![slot.clone()], false);

        self.emit_packet(
            &party_websocket_channel_name,
            &WebsocketChannelNamespace::Party,
            &GameServerUpdatePackets::CharacterUnequippedSlot(UnequipSlotRequest {
                character_id,
                slot,
            }),
            None,
        )
    }
}
