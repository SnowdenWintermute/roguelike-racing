use crate::websocket_server::game_server::{
    getters::{get_mut_game, get_mut_user},
    GameServer,
};
use common::{
    app_consts::error_messages,
    errors::AppError,
    game::getters::{get_mut_party, get_mut_player},
    items::equipment::EquipmentSlots,
    packets::{client_to_server::UnequipSlotRequest, server_to_client::GameServerUpdatePackets},
};

impl GameServer {
    pub fn unequip_slot_handler(
        &mut self,
        actor_id: u32,
        character_id: u32,
        slot: EquipmentSlots,
    ) -> Result<(), AppError> {
        let connected_user = get_mut_user(&mut self.sessions, actor_id)?;
        let username = connected_user.username.clone();
        let current_game_name =
            connected_user
                .current_game_name
                .clone()
                .ok_or_else(|| AppError {
                    error_type: common::errors::AppErrorTypes::ServerError,
                    message: error_messages::MISSING_GAME_REFERENCE.to_string(),
                })?;
        let game = get_mut_game(&mut self.games, &current_game_name)?;
        let player = get_mut_player(game, username.clone())?;
        let player_character_ids_option = player.character_ids.clone();
        let party_id = player.party_id.ok_or_else(|| AppError {
            error_type: common::errors::AppErrorTypes::ServerError,
            message: error_messages::MISSING_PARTY_REFERENCE.to_string(),
        })?;
        let party = get_mut_party(game, party_id)?;

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
        character.unequip_slots(&vec![slot.clone()]);

        self.emit_packet(
            &current_game_name,
            &GameServerUpdatePackets::CharacterUnequippedSlot(UnequipSlotRequest {
                character_id,
                slot,
            }),
            None,
        )
    }
}
