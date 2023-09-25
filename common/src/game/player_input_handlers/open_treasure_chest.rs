use crate::{
    adventuring_party::AdventuringParty, errors::AppError, game::id_generator::IdGenerator,
    items::Item,
};

pub fn open_treasure_chest(
    id_generator: &mut IdGenerator,
    adventuring_party: &mut AdventuringParty,
) -> Result<(), AppError> {
    let current_room = adventuring_party.current_room.as_mut().ok_or(AppError {
        error_type: crate::errors::AppErrorTypes::InvalidInput,
        message: "tried to open a treasure chest but no dungeon room was found".to_string(),
    })?;

    let treasure_chest = current_room.treasure_chest.as_mut().ok_or({
        AppError {
            error_type: crate::errors::AppErrorTypes::InvalidInput,
            message: "Tried to open a treasure chest but there was none".to_string(),
        }
    })?;

    treasure_chest.is_opened = true;
    if current_room.items.is_none() {
        current_room.items = Some(Vec::new());
    }

    current_room.items.as_mut().unwrap().push(Item::generate(
        id_generator,
        adventuring_party.current_floor as u16,
    ));

    Ok(())
}
