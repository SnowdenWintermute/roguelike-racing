use crate::combat::hp_change_source_types::HpChangeSource;
use crate::combat::hp_change_source_types::HpChangeSourceCategories;
use crate::combatants::combat_attributes::CombatAttributes;
use crate::items::equipment::one_handed_melee_weapons::OneHandedMeleeWeapons;
use crate::items::equipment::weapon_properties::WeaponProperties;
use crate::items::equipment::EquipmentProperties;
use crate::items::equipment::EquipmentTraits;
use crate::items::equipment::EquipmentTypes;
use crate::items::Item;
use crate::items::ItemProperties;
use crate::primatives::Range;
use std::collections::HashMap;

pub fn create_item_with_damage_increase_mods(
    min_base: u8,
    max_base: u8,
    percent: u8,
    flat: u16,
) -> Item {
    Item::new(
        1,
        "test_wep".to_string(),
        1,
        None,
        ItemProperties::Equipment(EquipmentProperties::new(
            EquipmentTypes::OneHandedMeleeWeapon(
                OneHandedMeleeWeapons::Stick,
                WeaponProperties {
                    damage_classifications: vec![HpChangeSource {
                        category: HpChangeSourceCategories::PhysicalDamage,
                        sub_category: None,
                        element: None,
                    }],
                    damage: Range::new(min_base, max_base),
                },
            ),
            None,
            HashMap::from([(CombatAttributes::Damage, flat)]),
            vec![],
            Some(vec![EquipmentTraits::DamagePercentage(percent)]),
        )),
    )
}
