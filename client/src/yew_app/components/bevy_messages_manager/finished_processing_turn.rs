use crate::yew_app::store::game_store::GameStore;
use common::app_consts::error_messages;
use common::errors::AppError;
use yewdux::Dispatch;

pub fn finished_processing_turn(
    game_dispatch: Dispatch<GameStore>,
    combatant_id: u32,
) -> Result<(), AppError> {
    game_dispatch.reduce_mut(|store| -> Result<(), AppError> {
        if let Some(battle_id) = store.current_battle_id {
            let game = store.game.as_mut().ok_or_else(|| AppError {
                error_type: common::errors::AppErrorTypes::ClientError,
                message: error_messages::GAME_NOT_FOUND.to_string(),
            })?;
            let (_, combatant_properties) = game.get_combatant_by_id(&combatant_id)?;
            // if they are dead, their turn tracker should already be removed
            if combatant_properties.hit_points > 0 {
                game.end_active_combatant_turn(battle_id)?;
            }

            // animating combatants have their buttons disabled
            store.combatants_animating.remove(&combatant_id);
        }
        Ok(())
    })
}
