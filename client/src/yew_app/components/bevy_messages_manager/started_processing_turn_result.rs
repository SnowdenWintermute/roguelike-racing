use crate::yew_app::store::game_store::GameStore;
use common::errors::AppError;
use yewdux::Dispatch;

pub fn started_processing_turn_result(
    game_dispatch: Dispatch<GameStore>,
    combatant_id: u32,
) -> Result<(), AppError> {
    game_dispatch.reduce_mut(|store| {
        let game = store.get_current_game_mut()?;
        let (_, combatant_properties) = game.get_mut_combatant_by_id(&combatant_id)?;
        combatant_properties.selected_combat_action = None;
        combatant_properties.combat_action_targets = None;
        Ok(())
    })
}
