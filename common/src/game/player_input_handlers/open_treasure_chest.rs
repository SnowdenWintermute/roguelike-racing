use crate::{
    adventuring_party::AdventuringParty, dungeon_rooms::DungeonRoom, errors::AppError, items::Item,
};

use super::id_generator::IdGenerator;

pub fn open_treasure_chest(
    id_generator: &mut IdGenerator,
    adventuring_party: &AdventuringParty,
    current_room: &mut DungeonRoom,
) -> Result<(), AppError> {
    let ref treasure_chest = current_room.treasure_chest.ok_or({
        AppError {
            error_type: crate::errors::AppErrorTypes::InvalidInput,
            message: "Tried to open a treasure chest but there was none".to_string(),
        }
    })?;

    treasure_chest.is_opened = true;
    if current_room.items.is_none() {
        current_room.items = Some(Vec::new());
    }

    current_room.items.unwrap().push(Item::generate(
        id_generator,
        adventuring_party.current_floor as u16,
    ));

    Ok(())
}
