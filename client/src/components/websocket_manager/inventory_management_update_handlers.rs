use common::{
    app_consts::error_messages, errors::AppError, game::getters::get_mut_party,
    packets::server_to_client::CharacterEquippedItemPacket,
};

use crate::store::game_store::GameStore;

pub fn handle_character_equipped_item(
    game_store: &mut GameStore,
    packet: CharacterEquippedItemPacket,
) -> Result<(), AppError> {
    let CharacterEquippedItemPacket {
        character_id,
        item_id,
        alt_slot,
    } = packet;
    let mut game = &mut game_store.game.as_mut().ok_or_else(|| AppError {
        error_type: common::errors::AppErrorTypes::ClientError,
        message: error_messages::GAME_NOT_FOUND.to_string(),
    })?;
    let party_id = game_store.current_party_id.ok_or_else(|| AppError {
        error_type: common::errors::AppErrorTypes::ClientError,
        message: error_messages::MISSING_PARTY_REFERENCE.to_string(),
    })?;
    let party = get_mut_party(&mut game, party_id)?;
    let character = party
        .characters
        .get_mut(&character_id)
        .ok_or_else(|| AppError {
            error_type: common::errors::AppErrorTypes::ClientError,
            message: error_messages::CHARACTER_NOT_FOUND.to_string(),
        })?;

    character.equip_item(item_id, alt_slot)?;

    Ok(())
}
