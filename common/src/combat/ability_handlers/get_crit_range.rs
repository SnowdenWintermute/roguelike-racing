use crate::app_consts::BASE_CRIT_MULTIPLIER;

#[cfg(test)]
#[test]
fn crit_range_test() {
    let bonus_multiplier_as_percent = 10;
    let min_value: f32 = 100.0;
    let max_value: f32 = 150.0;
    let (new_min, new_max) = get_crit_range(bonus_multiplier_as_percent, min_value, max_value);
    assert_eq!(new_min, 160.0);
    assert_eq!(new_max, 240.0);
}

pub fn get_crit_range(
    bonus_multiplier_as_percent: u16,
    min_value: f32,
    max_value: f32,
) -> (f32, f32) {
    let bonus_multiplier_as_float = bonus_multiplier_as_percent as f32 / 100.0;
    let crit_multiplier = BASE_CRIT_MULTIPLIER + bonus_multiplier_as_float;
    (min_value * crit_multiplier, max_value * crit_multiplier)
}
