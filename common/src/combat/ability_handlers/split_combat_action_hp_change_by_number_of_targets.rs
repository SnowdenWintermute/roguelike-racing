use crate::app_consts::MULTI_TARGET_HP_CHANGE_BONUS;

pub fn split_combat_action_hp_change_by_number_of_targets(
    min_hp_change: u16,
    max_hp_change: u16,
    num_targets: u8,
) -> (f32, f32) {
    let multi_target_bonus = 1.0 + (num_targets as f32 - 1.0) * MULTI_TARGET_HP_CHANGE_BONUS;

    let min_hp_change = min_hp_change as f32 * multi_target_bonus;
    let max_hp_change = max_hp_change as f32 * multi_target_bonus;
    // split hp_change between all targets
    (
        min_hp_change / num_targets as f32,
        max_hp_change / num_targets as f32,
    )
}

#[cfg(test)]
#[test]
fn split_combat_action_hp_change_by_number_of_targets_test() {
    let min_hp_change: u16 = 30;
    let max_hp_change: u16 = 300;
    let num_targets = 3;
    let (new_min, new_max) = split_combat_action_hp_change_by_number_of_targets(
        min_hp_change,
        max_hp_change,
        num_targets,
    );
    assert_eq!(new_min, 13.0);
    assert_eq!(new_max, 130.0);
    let min_hp_change: u16 = 20;
    let max_hp_change: u16 = 200;
    let num_targets = 2;
    let (new_min, new_max) = split_combat_action_hp_change_by_number_of_targets(
        min_hp_change,
        max_hp_change,
        num_targets,
    );
    assert_eq!(new_min, 11.5);
    assert_eq!(new_max, 115.0);
}
