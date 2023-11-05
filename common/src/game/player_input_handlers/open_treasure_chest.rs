use crate::{
    adventuring_party::AdventuringParty, errors::AppError, game::id_generator::IdGenerator,
    items::Item,
};

pub fn open_treasure_chest(
    id_generator: &mut IdGenerator,
    adventuring_party: &mut AdventuringParty,
) -> Result<(), AppError> {
    let current_room = &mut adventuring_party.current_room;

    let treasure_chest = current_room
        .treasure_chest
        .as_mut()
        .ok_or_else(|| AppError {
            error_type: crate::errors::AppErrorTypes::InvalidInput,
            message: "Tried to open a treasure chest but there was none".to_string(),
        })?;

    treasure_chest.is_opened = true;
    if current_room.items.is_none() {
        current_room.items = Some(Vec::new());
    }

    current_room.items.as_mut().unwrap().push(Item::generate(
        id_generator,
        adventuring_party.current_floor,
    ));

    Ok(())
}
