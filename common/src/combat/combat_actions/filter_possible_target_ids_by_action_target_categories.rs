use crate::errors::AppError;
use crate::game::RoguelikeRacerGame;

use super::TargetCategories;

pub fn filter_possible_target_ids_by_action_target_categories(
    game: &RoguelikeRacerGame,
    target_categories: &TargetCategories,
    ally_ids: Vec<u32>,
    opponent_ids_option: Option<Vec<u32>>,
) -> Result<(Vec<u32>, Option<Vec<u32>>), AppError> {
    todo!()
}
