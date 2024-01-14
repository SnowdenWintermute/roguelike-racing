use crate::store::game_store::GameStore;
use common::errors::AppError;
use common::game::RoguelikeRacerGame;
use yewdux::Dispatch;

pub fn game_full_update_handler(
    game_dispatch: Dispatch<GameStore>,
    update: Option<RoguelikeRacerGame>,
) -> Result<(), AppError> {
    game_dispatch.reduce_mut(|store| {
        store.game = update;
    });
    Ok(())
}
