use crate::app_consts::MULTI_TARGET_HP_CHANGE_BONUS;

pub fn split_combat_action_hp_change_by_number_of_targets(
    min_hp_change: f32,
    max_hp_change: f32,
    num_targets: f32,
) -> (f32, f32) {
    let multi_target_bonus = 1.0 + (num_targets - 1.0) * MULTI_TARGET_HP_CHANGE_BONUS;

    let min_hp_change = min_hp_change * multi_target_bonus;
    let max_hp_change = max_hp_change * multi_target_bonus;
    // split hp_change between all targets
    (min_hp_change / num_targets, max_hp_change / num_targets)
}

#[cfg(test)]
#[test]
fn split_combat_action_hp_change_by_number_of_targets_test() {
    let min_hp_change: f32 = 30.0;
    let max_hp_change: f32 = 300.0;
    let num_targets = 3.0;
    let (new_min, new_max) = split_combat_action_hp_change_by_number_of_targets(
        min_hp_change,
        max_hp_change,
        num_targets,
    );
    assert_eq!(new_min, 13.0);
    assert_eq!(new_max, 130.0);
    let min_hp_change: f32 = 20.0;
    let max_hp_change: f32 = 200.0;
    let num_targets = 2.0;
    let (new_min, new_max) = split_combat_action_hp_change_by_number_of_targets(
        min_hp_change,
        max_hp_change,
        num_targets,
    );
    assert_eq!(new_min, 11.5);
    assert_eq!(new_max, 115.0);
}
