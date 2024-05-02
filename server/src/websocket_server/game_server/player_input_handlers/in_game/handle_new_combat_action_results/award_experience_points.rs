use common::combatants::CombatantProperties;
const BASE_XP_PER_MONSTER: f32 = 30.0;
// const BASE_XP_PER_MONSTER: f32 = 110.0;
const BASE_XP_LEVEL_DIFF_MULTIPLIER: f32 = 0.25;

pub fn award_experience_points(
    combatant_properties: &mut CombatantProperties,
    defeated_monster_levels: &Vec<u8>,
    num_characters_alive: u8,
) -> u16 {
    let mut total_xp_to_award = 0;
    for monster_level in defeated_monster_levels {
        let base_xp = BASE_XP_PER_MONSTER / num_characters_alive as f32;
        let level_diff = combatant_properties.level as i16 - *monster_level as i16;
        let diff_multiplier = BASE_XP_LEVEL_DIFF_MULTIPLIER * level_diff.abs() as f32;
        let mut xp_to_award_for_this_monster = base_xp;

        if level_diff > 0 {
            xp_to_award_for_this_monster -= base_xp * diff_multiplier;
        } else if level_diff < 0 {
            xp_to_award_for_this_monster += base_xp * diff_multiplier;
        }
        total_xp_to_award += xp_to_award_for_this_monster as u16;
    }

    combatant_properties.experience_points.current += total_xp_to_award;
    total_xp_to_award
}

#[cfg(test)]
#[test]
fn award_experience_points_test() {
    use common::combatants::combatant_classes::CombatantClass;
    use common::combatants::combatant_species::CombatantSpecies;
    use common::combatants::CombatantControlledBy;
    use std::collections::HashMap;

    let mut character_combatant_properties = CombatantProperties::new(
        &CombatantClass::Warrior,
        &CombatantSpecies::Humanoid,
        HashMap::new(),
        CombatantControlledBy::Player("test_user".to_string()),
    );

    character_combatant_properties.level = 1;
    character_combatant_properties.experience_points.current = 0;
    let defeated_monster_levels = vec![1];
    award_experience_points(
        &mut character_combatant_properties,
        &defeated_monster_levels,
        3,
    );
    assert_eq!(character_combatant_properties.experience_points.current, 10);

    character_combatant_properties.level = 2;
    character_combatant_properties.experience_points.current = 0;
    let defeated_monster_levels = vec![1];
    award_experience_points(
        &mut character_combatant_properties,
        &defeated_monster_levels,
        3,
    );
    assert_eq!(character_combatant_properties.experience_points.current, 7);

    character_combatant_properties.level = 1;
    character_combatant_properties.experience_points.current = 0;
    let defeated_monster_levels = vec![3];
    award_experience_points(
        &mut character_combatant_properties,
        &defeated_monster_levels,
        3,
    );
    assert_eq!(character_combatant_properties.experience_points.current, 15);
}
