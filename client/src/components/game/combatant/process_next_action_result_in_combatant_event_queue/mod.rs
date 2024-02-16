mod queue_attack_animations;
mod queue_consumable_use_animations;
mod queue_fire_animations;
mod queue_melee_ability_animations;
mod queue_return_to_home_position_animations;
use self::queue_attack_animations::queue_attack_animations;
use self::queue_consumable_use_animations::queue_consumable_use_animations;
use self::queue_fire_animations::queue_fire_animations;
use self::queue_melee_ability_animations::queue_melee_ability_animations;
use crate::store::game_store::GameStore;
use common::app_consts::error_messages;
use common::combat::combat_actions::CombatAction;
use common::combat::ActionResult;
use common::combatants::abilities::CombatantAbilityNames;
use common::errors::AppError;
use gloo::console::log;
use yewdux::Dispatch;

pub fn process_next_action_result_in_combatant_event_queue(
    game_dispatch: Dispatch<GameStore>,
    current_action_processing: &Option<ActionResult>,
    combatant_id: u32,
) -> Result<(), AppError> {
    if let Some(new_action_result) = &current_action_processing {
        log!("processing next action result");
        game_dispatch.reduce_mut(|store| -> Result<(), AppError> {
            let event_manager = store
                .action_results_manager
                .combantant_event_managers
                .get_mut(&combatant_id)
                .ok_or_else(|| AppError {
                    error_type: common::errors::AppErrorTypes::ClientError,
                    message: error_messages::COMBANTANT_EVENT_MANAGER_NOT_FOUND.to_string(),
                })?;
            event_manager.last_processed_action_ended_turn = new_action_result.ends_turn;

            let game = store.get_current_game_mut()?;
            let (_, action_user_combatant_properties) =
                game.get_mut_combatant_by_id(&combatant_id)?;
            action_user_combatant_properties.selected_combat_action = None;
            action_user_combatant_properties.combat_action_targets = None;

            Ok(())
        })?;

        match &new_action_result.action {
            CombatAction::AbilityUsed(ability_name) => {
                match ability_name.get_attributes().is_melee {
                    true => queue_melee_ability_animations(
                        game_dispatch.clone(),
                        combatant_id,
                        new_action_result,
                    )?,
                    false => (),
                };
                match ability_name {
                    CombatantAbilityNames::Attack
                    | CombatantAbilityNames::AttackMeleeMainhand
                    | CombatantAbilityNames::AttackMeleeOffhand
                    | CombatantAbilityNames::AttackRangedMainhand => {
                        queue_attack_animations(game_dispatch, combatant_id, new_action_result)
                    }
                    CombatantAbilityNames::Fire => {
                        queue_fire_animations(game_dispatch, combatant_id, new_action_result)
                    }
                    CombatantAbilityNames::Healing => {
                        queue_fire_animations(game_dispatch, combatant_id, new_action_result)
                    }
                    _ => Ok(()),
                }
            }
            CombatAction::ConsumableUsed(item_id) => {
                queue_consumable_use_animations(
                    game_dispatch.clone(),
                    combatant_id,
                    new_action_result,
                )?;
                game_dispatch.reduce_mut(|store| -> Result<(), AppError> {
                    let game = store.get_current_game_mut()?;
                    let (_, action_user_combatant_properties) =
                        game.get_mut_combatant_by_id(&combatant_id)?;
                    let consumable = action_user_combatant_properties
                        .inventory
                        .get_consumable_mut(&item_id)?;
                    consumable.uses_remaining -= 1;
                    let should_be_removed_from_inventory = consumable.uses_remaining == 0;
                    if should_be_removed_from_inventory {
                        action_user_combatant_properties
                            .inventory
                            .remove_item(*item_id)?;
                    }
                    Ok(())
                })
            }
        }
    } else {
        let ends_turn = game_dispatch.reduce_mut(|store| -> Result<bool, AppError> {
            let event_manager = store
                .action_results_manager
                .combantant_event_managers
                .get_mut(&combatant_id)
                .ok_or_else(|| AppError {
                    error_type: common::errors::AppErrorTypes::ClientError,
                    message: error_messages::COMBANTANT_EVENT_MANAGER_NOT_FOUND.to_string(),
                })?;
            Ok(event_manager.last_processed_action_ended_turn)
        })?;
        queue_return_to_home_position_animations::queue_return_to_home_position_animations(
            game_dispatch,
            combatant_id,
            ends_turn,
        )
    }
}
