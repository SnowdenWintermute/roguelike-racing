use crate::adventuring_party::AdventuringParty;
use crate::errors::AppError;

pub fn use_selected_consumable(
    adventuring_party: &mut AdventuringParty,
    player_character_id: u32,
) -> Result<(), AppError> {
    let player_character = adventuring_party
        .characters
        .get(&player_character_id)
        .ok_or_else(|| AppError {
            error_type: crate::errors::AppErrorTypes::InvalidInput,
            message: "tried to process player input but couldn't find the player character"
                .to_string(),
        })?;

    let selected_item_slot = player_character
        .combatant_properties
        .selected_item_slot
        .ok_or_else(|| AppError {
            error_type: crate::errors::AppErrorTypes::InvalidInput,
            message: "Tried to use the selected item but no item was selected".to_string(),
        })?;

    let _selected_consumable = player_character
        .inventory
        .items
        .get(selected_item_slot as usize)
        .ok_or_else(|| AppError {
            error_type: crate::errors::AppErrorTypes::InvalidInput,
            message: "Tried to select an item but no item found in the inventory slot".to_string(),
        })?;
    // based on targets and consumable type, do something complicated
    Ok(())
}
