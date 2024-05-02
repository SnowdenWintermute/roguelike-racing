use common::combat::apply_action_result::apply_action_result;
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
