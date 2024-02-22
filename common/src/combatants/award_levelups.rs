use super::combat_attributes::CombatAttributes;
use super::CombatantProperties;

const XP_REQUIRED_TO_LEVEL_INCREASE_INCREMENT: u16 = 25;
const ABILITY_POINTS_AWARDED_PER_LEVEL: u8 = 2;
const ATTRIBUTE_POINTS_AWARDED_PER_LEVEL: u8 = 5;

pub fn award_levelups(combatant_properties: &mut CombatantProperties) {
    let mut calculating_new_levelups = true;
    while calculating_new_levelups {
        if let Some(required_to_level) = combatant_properties
            .experience_points
            .required_for_next_level
        {
            if combatant_properties.experience_points.current >= required_to_level {
                combatant_properties.level += 1;
                combatant_properties.unspent_ability_points += ABILITY_POINTS_AWARDED_PER_LEVEL;
                combatant_properties.unspent_attribute_points += ATTRIBUTE_POINTS_AWARDED_PER_LEVEL;
                combatant_properties.experience_points.current -= required_to_level;

                let combatant_attributes = combatant_properties.get_total_attributes();
                combatant_properties.hit_points = *combatant_attributes
                    .get(&CombatAttributes::Hp)
                    .unwrap_or_else(|| &0);
                combatant_properties.mana = *combatant_attributes
                    .get(&CombatAttributes::Mp)
                    .unwrap_or_else(|| &0);

                combatant_properties
                    .experience_points
                    .required_for_next_level =
                    Some(required_to_level + XP_REQUIRED_TO_LEVEL_INCREASE_INCREMENT);
            } else {
                calculating_new_levelups = false;
            }
        } else {
            calculating_new_levelups = false;
        }
    }
}

#[cfg(test)]
#[test]
fn award_levelups_test() {
    use crate::combatants::combatant_classes::CombatantClass;
    use crate::combatants::CombatantControlledBy;
    use std::collections::HashMap;

    let mut combatant_properties = CombatantProperties::new(
        &CombatantClass::Warrior,
        HashMap::new(),
        CombatantControlledBy::Player("test_user".to_string()),
    );

    combatant_properties.experience_points.current = 100;
    award_levelups(&mut combatant_properties);
    assert_eq!(combatant_properties.level, 2);
    assert_eq!(combatant_properties.experience_points.current, 0);
    assert_eq!(
        combatant_properties
            .experience_points
            .required_for_next_level,
        Some(125)
    );

    combatant_properties.level = 1;
    combatant_properties.experience_points.current = 300;
    combatant_properties
        .experience_points
        .required_for_next_level = Some(100);
    award_levelups(&mut combatant_properties);
    assert_eq!(combatant_properties.level, 3);
    assert_eq!(combatant_properties.experience_points.current, 75);
    assert_eq!(
        combatant_properties
            .experience_points
            .required_for_next_level,
        Some(150)
    );
}
