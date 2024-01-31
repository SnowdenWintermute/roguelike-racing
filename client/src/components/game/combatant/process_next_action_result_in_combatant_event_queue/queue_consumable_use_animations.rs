use crate::store::game_store::GameStore;
use common::app_consts::error_messages;
use common::combat::combat_actions::CombatAction;
use common::combat::ActionResult;
use common::errors::AppError;
use common::items::consumables::ConsumableTypes;
use yewdux::Dispatch;

pub fn queue_consumable_use_animations(
    game_dispatch: Dispatch<GameStore>,
    combatant_id: u32,
    action_result: &ActionResult,
) -> Result<(), AppError> {
    game_dispatch.reduce_mut(|game_store| -> Result<(), AppError> {
        let game = game_store.get_current_game()?;
        let (_, consumable_user_combatant_properties) = game.get_combatant_by_id(&combatant_id)?;
        let item_id = match action_result.action {
            CombatAction::AbilityUsed(_) => {
                return Err(AppError {
                    error_type: common::errors::AppErrorTypes::Generic,
                    message: error_messages::INVALID_ACTION_TYPE.to_string(),
                })
            }
            CombatAction::ConsumableUsed(id) => id,
        };
        let consumable = consumable_user_combatant_properties
            .inventory
            .get_consumable(&item_id)?;
        match consumable.consumable_type {
            ConsumableTypes::HpAutoinjector => todo!(),
            ConsumableTypes::Grenade => todo!(),
            ConsumableTypes::SmokeBomb => todo!(),
        }
    })
}
