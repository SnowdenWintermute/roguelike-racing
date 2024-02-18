use crate::combatants::CombatantProperties;
use crate::errors::AppError;
use crate::items::equipment::EquipmentSlots;
use crate::primatives::WeaponSlot;

pub fn add_weapon_damage_to_combat_action_hp_change(
    weapon_slots: &Vec<WeaponSlot>,
    user_combatant_properties: &CombatantProperties,
    min: &f32,
    max: &f32,
) -> Result<(f32, f32), AppError> {
    let mut min: f32 = *min;
    let mut max: f32 = *max;
    for weapon_slot in weapon_slots {
        match weapon_slot {
            WeaponSlot::MainHand => calculate_and_add_weapon_damage(
                user_combatant_properties,
                &EquipmentSlots::MainHand,
                &mut min,
                &mut max,
            )?,
            WeaponSlot::OffHand => calculate_and_add_weapon_damage(
                user_combatant_properties,
                &EquipmentSlots::OffHand,
                &mut min,
                &mut max,
            )?,
        }
    }

    Ok((min, max))
}

pub fn calculate_and_add_weapon_damage(
    user_combatant_properties: &CombatantProperties,
    slot: &EquipmentSlots,
    min: &mut f32,
    max: &mut f32,
) -> Result<(), AppError> {
    let equipment_option = user_combatant_properties.get_weapon_in_slot(slot);
    if let Some(equipment_properties) = equipment_option {
        println!("found equipment_properties in slot {:#?}", slot);
        let (weapon_min, weapon_max) = equipment_properties.get_modified_weapon_damage_range()?;
        println!("weapon min: {weapon_min} weapon max: {weapon_max}");
        *min += weapon_min;
        *max += weapon_max;
    }
    Ok(())
}

#[cfg(test)]
#[test]

pub fn add_weapon_damage_to_combat_action_hp_change_test() -> Result<(), AppError> {
    use crate::combatants::CombatantClass;
    use crate::combatants::CombatantControlledBy;
    use crate::tests::test_items::test_items::create_item_with_damage_increase_mods;
    use std::collections::HashMap;

    let mut user_combatant_properties = CombatantProperties::new(
        &CombatantClass::Warrior,
        HashMap::new(),
        CombatantControlledBy::Player("".to_string()),
    );
    let weapon_slots = vec![(WeaponSlot::MainHand)];
    let min = 0.0;
    let max = 0.0;
    let (new_min, new_max) = add_weapon_damage_to_combat_action_hp_change(
        &weapon_slots,
        &user_combatant_properties,
        &min,
        &max,
    )?;
    assert_eq!(new_min, 1.0);
    assert_eq!(new_max, 1.0);

    user_combatant_properties.equipment.insert(
        EquipmentSlots::MainHand,
        create_item_with_damage_increase_mods(10, 100, 50, 0),
    );
    let (new_min, new_max) = add_weapon_damage_to_combat_action_hp_change(
        &weapon_slots,
        &user_combatant_properties,
        &min,
        &max,
    )?;
    assert_eq!(new_min, 15.0);
    assert_eq!(new_max, 150.0);

    user_combatant_properties.equipment.insert(
        EquipmentSlots::MainHand,
        create_item_with_damage_increase_mods(10, 100, 50, 10),
    );
    let (new_min, new_max) = add_weapon_damage_to_combat_action_hp_change(
        &weapon_slots,
        &user_combatant_properties,
        &min,
        &max,
    )?;
    assert_eq!(new_min, 30.0);
    assert_eq!(new_max, 165.0);

    Ok(())
}
