use common::app_consts::error_messages;
use common::combat::combat_actions::CombatAction;
use common::combat::ActionResult;
use common::errors::AppError;
use common::game::RoguelikeRacerGame;

pub fn apply_action_results(
    game: &mut RoguelikeRacerGame,
    action_results: &Vec<ActionResult>,
    battle_id_option: Option<u32>,
) -> Result<(), AppError> {
    for action_result in action_results {
        apply_action_result(game, &action_result, battle_id_option)?;
    }

    Ok(())
}

pub fn apply_action_result(
    game: &mut RoguelikeRacerGame,
    action_result: &ActionResult,
    battle_id_option: Option<u32>,
) -> Result<(), AppError> {
    let user_id = action_result.user_id;
    let (_, user_combatant_properties) = game.get_mut_combatant_by_id(&user_id)?;
    user_combatant_properties.selected_ability_name = None;
    user_combatant_properties.selected_consumable = None;
    user_combatant_properties.combat_action_targets = None;

    match action_result.action {
        CombatAction::AbilityUsed(_) => (),
        CombatAction::ConsumableUsed(item_id) => {
            let consumable = user_combatant_properties
                .inventory
                .get_consumable_mut(&item_id)?;
            consumable.uses_remaining -= 1;
            let should_be_removed_from_inventory = consumable.uses_remaining == 0;
            if should_be_removed_from_inventory {
                user_combatant_properties.inventory.remove_item(item_id)?;
            }
        }
    }

    if let Some(hp_changes) = &action_result.hp_changes_by_entity_id {
        for (entity_id, hp_change) in hp_changes.iter() {
            let (entity_properties, combatant_properties) =
                game.get_mut_combatant_by_id(entity_id)?;
            let entity_id = entity_properties.id;
            combatant_properties.change_hp(*hp_change);

            if combatant_properties.hit_points == 0 {
                if let Some(battle_id) = battle_id_option {
                    let battle = game.battles.get_mut(&battle_id).ok_or_else(|| AppError {
                        error_type: common::errors::AppErrorTypes::ServerError,
                        message: error_messages::BATTLE_NOT_FOUND.to_string(),
                    })?;
                    let mut index_to_remove_option = None;
                    for (i, turn_tracker) in battle.combatant_turn_trackers.iter().enumerate() {
                        if turn_tracker.entity_id == entity_id {
                            index_to_remove_option = Some(i)
                        }
                    }
                    if let Some(index_to_remove) = index_to_remove_option {
                        let _ = battle.combatant_turn_trackers.remove(index_to_remove);
                    }
                }
            }
        }
    }

    if let Some(mp_changes) = &action_result.mp_changes_by_entity_id {
        for (entity_id, mp_change) in mp_changes.iter() {
            let (_, combatant_properties) = game.get_mut_combatant_by_id(entity_id)?;
            combatant_properties.change_mp(*mp_change);
        }
    }

    Ok(())
}
