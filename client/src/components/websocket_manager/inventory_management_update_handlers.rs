use crate::store::game_store::{DetailableEntities, GameStore};
use common::{
    app_consts::error_messages, errors::AppError, game::getters::get_mut_party,
    packets::server_to_client::CharacterEquippedItemPacket,
};
use yewdux::prelude::Dispatch;

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

    let unequipped_item_ids = character.equip_item(item_id, alt_slot)?;
    let item_to_select = match unequipped_item_ids.get(0) {
        Some(id) => {
            let mut item = None;
            for item_in_inventory in &character.inventory.items {
                if item_in_inventory.entity_properties.id == *id {
                    item = Some(item_in_inventory)
                }
            }
            item
        }
        None => None,
    };

    match item_to_select {
        Some(item) => {
            game_store.selected_item = Some(item.clone());
            game_store.detailed_entity = Some(DetailableEntities::Item(item.clone()));
            game_store.hovered_entity = None;
        }
        None => (),
    }

    Ok(())
}
