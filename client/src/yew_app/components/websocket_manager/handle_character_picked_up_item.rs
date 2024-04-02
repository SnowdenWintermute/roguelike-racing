use crate::yew_app::store::game_store::GameStore;
use common::errors::AppError;
use common::packets::CharacterAndItem;
use yewdux::Dispatch;

pub fn handle_character_picked_up_item(
    game_dispatch: Dispatch<GameStore>,
    packet: CharacterAndItem,
) -> Result<(), AppError> {
    game_dispatch.reduce_mut(|store| -> Result<(), AppError> {
        let party = store.get_current_party_mut()?;
        let item_picked_up = party.remove_item_from_ground(packet.item_id)?;
        let character = store.get_mut_character(packet.character_id)?;
        character.combatant_properties.inventory.items.push(item_picked_up);

        Ok(())
    })
}
