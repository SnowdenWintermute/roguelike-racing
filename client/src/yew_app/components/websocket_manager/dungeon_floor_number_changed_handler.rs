use common::errors::AppError;
use yewdux::Dispatch;

use crate::yew_app::store::game_store::GameStore;

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
