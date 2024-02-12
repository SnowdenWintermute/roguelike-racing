use crate::components::mesh_manager::AutoinjectorTypes;
use crate::components::mesh_manager::CombatantAnimation;
use crate::store::game_store::GameStore;
use common::app_consts::error_messages;
use common::combat::combat_actions::CombatAction;
use common::combat::ActionResult;
use common::errors::AppError;
use common::items::consumables::ConsumableTypes;
use std::collections::VecDeque;
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
        let target_id = action_result.targets.get_single_target_id()?;

        match consumable.consumable_type {
            ConsumableTypes::HpAutoinjector => {
                let hp_changes =
                    &action_result
                        .hp_changes_by_entity_id
                        .as_ref()
                        .ok_or_else(|| AppError {
                            error_type: common::errors::AppErrorTypes::Generic,
                            message: error_messages::MISSING_EXPECTED_ACTION_RESULT_DATA
                                .to_string(),
                        })?;
                let hp_change = hp_changes.get(&target_id).ok_or_else(|| AppError {
                    error_type: common::errors::AppErrorTypes::Generic,
                    message: error_messages::MISSING_EXPECTED_ACTION_RESULT_DATA.to_string(),
                })?;
                let hp_change = *hp_change;

                let event_manager = game_store
                    .action_results_manager
                    .combantant_event_managers
                    .get_mut(&combatant_id)
                    .ok_or_else(|| AppError {
                        error_type: common::errors::AppErrorTypes::ClientError,
                        message: error_messages::COMBANTANT_EVENT_MANAGER_NOT_FOUND.to_string(),
                    })?;

                event_manager.animation_queue.append(&mut VecDeque::from([
                    CombatantAnimation::UseAutoinjector(
                        AutoinjectorTypes::Hp,
                        target_id,
                        hp_change,
                    ),
                ]));
                Ok(())
            }
            ConsumableTypes::MpAutoinjector => {
                let mp_changes =
                    &action_result
                        .mp_changes_by_entity_id
                        .as_ref()
                        .ok_or_else(|| AppError {
                            error_type: common::errors::AppErrorTypes::Generic,
                            message: error_messages::MISSING_EXPECTED_ACTION_RESULT_DATA
                                .to_string(),
                        })?;
                let mp_change = mp_changes.get(&target_id).ok_or_else(|| AppError {
                    error_type: common::errors::AppErrorTypes::Generic,
                    message: error_messages::MISSING_EXPECTED_ACTION_RESULT_DATA.to_string(),
                })?;
                let mp_change = *mp_change;

                let event_manager = game_store
                    .action_results_manager
                    .combantant_event_managers
                    .get_mut(&combatant_id)
                    .ok_or_else(|| AppError {
                        error_type: common::errors::AppErrorTypes::ClientError,
                        message: error_messages::COMBANTANT_EVENT_MANAGER_NOT_FOUND.to_string(),
                    })?;

                event_manager.animation_queue.append(&mut VecDeque::from([
                    CombatantAnimation::UseAutoinjector(
                        AutoinjectorTypes::Mp,
                        target_id,
                        mp_change,
                    ),
                ]));
                Ok(())
            }
            ConsumableTypes::Grenade => todo!(),
            ConsumableTypes::SmokeBomb => todo!(),
        }
    })
}
