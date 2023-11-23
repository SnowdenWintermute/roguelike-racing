use crate::store::game_store::{DetailableEntities, GameStore};
use common::{
    app_consts::error_messages,
    character::Character,
    errors::AppError,
    game::getters::get_mut_party,
    packets::{
        client_to_server::UnequipSlotRequest, server_to_client::CharacterEquippedItemPacket,
    },
};

pub fn handle_character_equipped_item(
    game_store: &mut GameStore,
    packet: CharacterEquippedItemPacket,
) -> Result<(), AppError> {
    let CharacterEquippedItemPacket {
        character_id,
        item_id,
        alt_slot,
    } = packet;
    let character = game_store.get_mut_character(character_id)?;

    let unequipped_item_ids = character.equip_item(item_id, alt_slot)?;
    let item_to_select = match unequipped_item_ids.get(0) {
        Some(id) => {
            let mut item = None;
            for item_in_inventory in &character.inventory.items {
                if item_in_inventory.entity_properties.id == *id {
                    item = Some(item_in_inventory.clone())
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

pub fn handle_character_unequipped_slot(
    game_store: &mut GameStore,
    packet: UnequipSlotRequest,
) -> Result<(), AppError> {
    let UnequipSlotRequest { character_id, slot } = packet;
    let character = game_store.get_mut_character(character_id)?;
    character.unequip_slots(&vec![slot], false);
    Ok(())
}

impl GameStore {
    pub fn get_mut_character<'a>(
        &'a mut self,
        character_id: u32,
    ) -> Result<&'a mut Character, AppError> {
        let game = self.game.as_mut().ok_or_else(|| AppError {
            error_type: common::errors::AppErrorTypes::ClientError,
            message: error_messages::GAME_NOT_FOUND.to_string(),
        })?;
        let party_id = self.current_party_id.ok_or_else(|| AppError {
            error_type: common::errors::AppErrorTypes::ClientError,
            message: error_messages::MISSING_PARTY_REFERENCE.to_string(),
        })?;
        let party = get_mut_party(game, party_id)?;
        party
            .characters
            .get_mut(&character_id)
            .ok_or_else(|| AppError {
                error_type: common::errors::AppErrorTypes::ClientError,
                message: error_messages::CHARACTER_NOT_FOUND.to_string(),
            })
    }
}
