use super::TargetCategories;

pub fn filter_possible_target_ids_by_action_target_categories(
    target_categories: &TargetCategories,
    action_user_id: u32,
    ally_ids: Vec<u32>,
    opponent_ids_option: Option<Vec<u32>>,
) -> (Option<Vec<u32>>, Option<Vec<u32>>) {
    match target_categories {
        TargetCategories::Opponent => (None, opponent_ids_option),
        TargetCategories::User => (Some(vec![action_user_id]), None),
        TargetCategories::Friendly => (Some(ally_ids), None),
        TargetCategories::Any => (Some(ally_ids), opponent_ids_option),
    }
}
