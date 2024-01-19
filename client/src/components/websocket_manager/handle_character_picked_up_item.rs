use crate::store::game_store::GameStore;
use common::errors::AppError;
use common::packets::CharacterPickedUpItemPacket;
use gloo::console::log;
use yewdux::Dispatch;

pub fn handle_character_picked_up_item(
    game_dispatch: Dispatch<GameStore>,
    packet: CharacterPickedUpItemPacket,
) -> Result<(), AppError> {
    game_dispatch.reduce_mut(|store| -> Result<(), AppError> {
        log!(format!(
            "character {} picked up item {}",
            packet.character_id, packet.item_id
        ));
        let party = store.get_current_party_mut()?;
        let item_picked_up = party.remove_item_from_ground(packet.item_id)?;
        log!(format!(
            "removing item {}",
            item_picked_up.entity_properties.name
        ));
        let character = store.get_mut_character(packet.character_id)?;
        character.inventory.items.push(item_picked_up);

        Ok(())
    })
}
