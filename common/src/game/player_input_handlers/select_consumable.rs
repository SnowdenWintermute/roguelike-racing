use crate::character::Character;
use crate::errors::AppError;
use crate::items::ItemCategories;

pub fn select_consumable(
    player_character: &mut Character,
    inventory_slot: u8,
) -> Result<(), AppError> {
    let selected_consumable = player_character
        .inventory
        .items
        .get(inventory_slot as usize)
        .ok_or_else(||AppError {
            error_type: crate::errors::AppErrorTypes::InvalidInput,
            message: "Tried to select an item but no item found in the inventory slot".to_string(),
        })?;

    if selected_consumable.item_category != ItemCategories::Consumable {
        return Err(AppError {
            error_type: crate::errors::AppErrorTypes::InvalidInput,
            message: "Can't select a non-consumable item".to_string(),
        });
    }
    player_character.combatant_properties.selected_item_slot = Some(inventory_slot);

    Ok(())
}
