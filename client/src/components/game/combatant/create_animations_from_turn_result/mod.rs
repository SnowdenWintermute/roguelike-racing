use crate::store::game_store::GameStore;
use common::combat::CombatTurnResult;
use std::collections::VecDeque;
use yewdux::Dispatch;

pub fn create_animations_from_turn_result(
    game_dispatch: Dispatch<GameStore>,
    turn_result: CombatTurnResult,
) {
    let mut action_results = VecDeque::from(turn_result.action_results);
    while action_results.len() > 0 {
        let current_action_result = action_results.pop_front();
    }
}
