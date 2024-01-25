use crate::store::game_store::GameStore;
use common::errors::AppError;
use common::game::getters::get_mut_party;
use yewdux::Dispatch;

pub fn dungeon_floor_number_changed_handler(
    game_dispatch: Dispatch<GameStore>,
    packet: u8,
) -> Result<(), AppError> {
    game_dispatch.reduce_mut(|store| {
        let party = store.get_current_party_mut()?;
        party.current_floor = packet;
        Ok(())
    })
}
