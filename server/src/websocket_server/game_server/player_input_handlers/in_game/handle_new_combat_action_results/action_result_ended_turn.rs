use common::combat::ActionResult;

pub fn action_result_ended_turn(
    battle_id_option: Option<u32>,
    action_results: &Vec<ActionResult>,
) -> bool {
    if battle_id_option.is_some() {
        let mut should_end = false;
        for action_result in action_results {
            if action_result.ends_turn {
                should_end = true;
                break;
            }
        }
        should_end
    } else {
        false
    }
}
