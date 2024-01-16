use crate::store::game_store::GameStore;
use common::errors::AppError;
use yewdux::Dispatch;

pub fn handle_return_to_ready_position_animation_finished(
    game_dipatch: Dispatch<GameStore>,
) -> Result<(), AppError> {
    // if in battle, call for next turn result to be passed to it's enitity
    Ok(())
}
