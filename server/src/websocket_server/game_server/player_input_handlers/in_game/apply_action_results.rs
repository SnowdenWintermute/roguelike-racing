use common::combat::ActionResult;
use common::errors::AppError;
use common::game::RoguelikeRacerGame;

pub fn apply_action_results(
    game: &mut RoguelikeRacerGame,
    action_results: &Vec<ActionResult>,
) -> Result<(), AppError> {
    for action_result in action_results {
        apply_action_result(game, &action_result)?;
    }

    Ok(())
}

pub fn apply_action_result(
    game: &mut RoguelikeRacerGame,
    action_result: &ActionResult,
) -> Result<(), AppError> {
    if let Some(hp_changes) = &action_result.hp_changes_by_entity_id {
        for (entity_id, hp_change) in hp_changes.iter() {
            let (_, combatant_properties) = game.get_mut_combatant_by_id(entity_id)?;
            combatant_properties.change_hp(*hp_change);
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
