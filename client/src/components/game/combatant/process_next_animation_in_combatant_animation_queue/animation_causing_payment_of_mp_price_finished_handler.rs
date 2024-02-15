use crate::store::game_store::GameStore;
use common::errors::AppError;
use yewdux::Dispatch;

pub fn animation_causing_payment_of_mp_price_finished_handler(
    game_dispatch: Dispatch<GameStore>,
    mp_price: u8,
    causer_id: u32,
) -> Result<(), AppError> {
    game_dispatch.reduce_mut(|store| -> Result<(), AppError> {
        let game = store.get_current_game_mut()?;
        let (_, causer_combatant_properties) = game.get_mut_combatant_by_id(&causer_id)?;
        causer_combatant_properties.change_mp(mp_price as i16 * -1);
        Ok(())
    })
}
