use crate::app_consts::error_messages;
use crate::combatants::abilities::CombatantAbility;
use crate::errors::AppError;

#[cfg(test)]
#[test]
fn get_ability_base_hp_change_range_test() -> Result<(), AppError> {
    use crate::combatants::abilities::CombatantAbilityNames;

    let mut ability = CombatantAbility {
        ability_name: CombatantAbilityNames::Fire,
        level: 1,
    };

    let (min_base_hp_change, max_base_hp_change) = get_ability_base_hp_change_range(&ability, 10)?;
    assert_eq!(min_base_hp_change, 11);
    assert_eq!(max_base_hp_change, 15);
    ability.level = 2;
    let (min_base_hp_change, max_base_hp_change) = get_ability_base_hp_change_range(&ability, 10)?;
    assert_eq!(min_base_hp_change, 12);
    assert_eq!(max_base_hp_change, 20);
    Ok(())
}

pub fn get_ability_base_hp_change_range(
    ability: &CombatantAbility,
    scaling_attribute: u16,
) -> Result<(u16, u16), AppError> {
    let base_hp_change_range = ability
        .ability_name
        .get_attributes()
        .combat_action_properties
        .hp_change_properties
        .ok_or_else(|| AppError {
            error_type: crate::errors::AppErrorTypes::Generic,
            message: error_messages::MISSING_ACTION_HP_CHANGE_BASE_VALUES.to_string(),
        })?
        .base_values;

    let min_base_hp_change = (ability.level as u16 * base_hp_change_range.min) + scaling_attribute;
    let max_base_hp_change = (ability.level as u16 * base_hp_change_range.max) + scaling_attribute;

    Ok((min_base_hp_change, max_base_hp_change))
}
