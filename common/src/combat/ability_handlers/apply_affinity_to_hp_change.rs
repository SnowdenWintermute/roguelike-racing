pub fn apply_affinity_to_hp_change(affinity_as_percentage: i16, hp_change: f32) -> f32 {
    let affinity_multiplier = if affinity_as_percentage < 0 {
        1.0 + (affinity_as_percentage as f32 * -1.0) / 100.0
    } else if affinity_as_percentage > 0 && affinity_as_percentage <= 100 {
        1.0 - affinity_as_percentage as f32 / 100.0
    } else if affinity_as_percentage > 100 {
        let capped = std::cmp::min(affinity_as_percentage, 200);
        (capped as f32 - 100.0) / 100.0 * -1.0
    } else {
        1.0 as f32
    };

    affinity_multiplier * hp_change
}

#[cfg(test)]
#[test]
fn apply_elemental_affinity_to_hp_change_range_test() {
    // let min_damage = 10.0;
    // let max_damage = 100.0;
    // let affinity_as_percentage: i16 = 0;
    // let (new_min, new_max) =
    //     apply_elemental_affinity_to_hp_change_range(affinity_as_percentage, min_damage, max_damage);
    // assert_eq!(new_min, 10.0);
    // assert_eq!(new_max, 100.0);
    // let affinity_as_percentage: i16 = 100;
    // let (new_min, new_max) =
    //     apply_elemental_affinity_to_hp_change_range(affinity_as_percentage, min_damage, max_damage);
    // assert_eq!(new_min, 0.0);
    // assert_eq!(new_max, 0.0);
    // let affinity_as_percentage: i16 = 50;
    // let (new_min, new_max) =
    //     apply_elemental_affinity_to_hp_change_range(affinity_as_percentage, min_damage, max_damage);
    // assert_eq!(new_min, 5.0);
    // assert_eq!(new_max, 50.0);
    // let affinity_as_percentage: i16 = -50;
    // let (new_min, new_max) =
    //     apply_elemental_affinity_to_hp_change_range(affinity_as_percentage, min_damage, max_damage);
    // assert_eq!(new_min, 15.0);
    // assert_eq!(new_max, 150.0);
    // let affinity_as_percentage: i16 = -100;
    // let (new_min, new_max) =
    //     apply_elemental_affinity_to_hp_change_range(affinity_as_percentage, min_damage, max_damage);
    // assert_eq!(new_min, 20.0);
    // assert_eq!(new_max, 200.0);
    // let affinity_as_percentage: i16 = -200;
    // let (new_min, new_max) =
    //     apply_elemental_affinity_to_hp_change_range(affinity_as_percentage, min_damage, max_damage);
    // assert_eq!(new_min, 30.0);
    // assert_eq!(new_max, 300.0);
    // let affinity_as_percentage: i16 = 150;
    // let (new_min, new_max) =
    //     apply_elemental_affinity_to_hp_change_range(affinity_as_percentage, min_damage, max_damage);
    // assert_eq!(new_min, -5.0);
    // assert_eq!(new_max, -50.0);
    // let affinity_as_percentage: i16 = 250;
    // let (new_min, new_max) =
    //     apply_elemental_affinity_to_hp_change_range(affinity_as_percentage, min_damage, max_damage);
    // assert_eq!(new_min, -10.0);
    // assert_eq!(new_max, -100.0);
}
