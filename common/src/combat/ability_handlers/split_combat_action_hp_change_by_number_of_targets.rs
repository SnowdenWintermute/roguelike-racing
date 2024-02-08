use crate::app_consts::MULTI_TARGET_HP_CHANGE_BONUS;

pub fn split_combat_action_hp_change_by_number_of_targets(hp_change: f32, num_targets: f32) -> f32 {
    let multi_target_bonus = 1.0 + (num_targets - 1.0) * MULTI_TARGET_HP_CHANGE_BONUS;

    let hp_change = hp_change * multi_target_bonus;
    // split hp_change between all targets
    hp_change / num_targets
}

#[cfg(test)]
#[test]
fn split_combat_action_hp_change_by_number_of_targets_test() {
    let hp_change: f32 = 300.0;
    let num_targets = 3.0;
    let new_change = split_combat_action_hp_change_by_number_of_targets(hp_change, num_targets);
    assert_eq!(new_change, 130.0);
    let hp_change: f32 = 100.0;
    let num_targets = 2.0;
    let new_change = split_combat_action_hp_change_by_number_of_targets(hp_change, num_targets);
    assert_eq!(new_change, 57.5);
}
