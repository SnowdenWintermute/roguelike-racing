use crate::store::game_store::GameStore;
use common::errors::AppError;
use yewdux::Dispatch;

pub fn handle_follow_through_swing_animation_finished(
    game_dispatch: Dispatch<GameStore>,
) -> Result<(), AppError> {
    // pop the current action and query the next one
    Ok(())
}
