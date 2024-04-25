use crate::yew_app::store::game_store::GameStore;
use common::errors::AppError;
use yewdux::Dispatch;

pub fn mp_change_message_handler(
    game_dispatch: Dispatch<GameStore>,
    target_id: u32,
    mp_change: i16,
) -> Result<(), AppError> {
    game_dispatch.reduce_mut(|store| -> Result<(), AppError> {
        let game = store.get_current_game_mut()?;
        let (_, combatant_properties) = game.get_mut_combatant_by_id(&target_id)?;
        combatant_properties.change_mp(mp_change);

        Ok(())
    })
}
