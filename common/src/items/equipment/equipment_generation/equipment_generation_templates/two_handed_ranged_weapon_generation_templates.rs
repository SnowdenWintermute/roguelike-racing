use super::generate_templates::generate_templates;
use super::WeaponGenerationTemplate;
use crate::combat::hp_change_source_types::Evadable;
use crate::combat::hp_change_source_types::HpChangeSource;
use crate::combat::hp_change_source_types::HpChangeSourceCategories;
use crate::combat::hp_change_source_types::MeleeOrRanged;
use crate::combat::hp_change_source_types::PhysicalDamageTypes;
use crate::combatants::combat_attributes::CombatAttributes;
use crate::items::equipment::two_handed_ranged_weapons::TwoHandedRangedWeapons;
use crate::items::items_by_level::items_by_level;
use crate::primatives::Range;
use once_cell::sync::Lazy;
use std::collections::HashMap;

fn two_handed_ranged_weapon_template_from_base_item(
    item: &TwoHandedRangedWeapons,
    mut requirements: HashMap<CombatAttributes, u8>,
) -> WeaponGenerationTemplate {
    match item {
        TwoHandedRangedWeapons::ShortBow => {
            requirements.insert(CombatAttributes::Dexterity, 5);
            WeaponGenerationTemplate::new(
                Range::new(1, 4),
                Range::new(2, 7),
                Some(1),
                vec![HpChangeSource::new(
                    HpChangeSourceCategories::PhysicalDamage(MeleeOrRanged::Ranged),
                    Some(PhysicalDamageTypes::Piercing),
                    None,
                )],
                1,
                Some(requirements),
                None,
                None,
            )
        }
        TwoHandedRangedWeapons::RecurveBow => {
            requirements.insert(CombatAttributes::Dexterity, 15);
            WeaponGenerationTemplate::new(
                Range::new(3, 6),
                Range::new(5, 10),
                Some(1),
                vec![HpChangeSource::new(
                    HpChangeSourceCategories::PhysicalDamage(MeleeOrRanged::Ranged),
                    Some(PhysicalDamageTypes::Piercing),
                    None,
                )],
                1,
                Some(requirements),
                None,
                None,
            )
        }
        TwoHandedRangedWeapons::CompositeBow => {
            requirements.insert(CombatAttributes::Dexterity, 35);
            WeaponGenerationTemplate::new(
                Range::new(5, 8),
                Range::new(8, 16),
                Some(1),
                vec![HpChangeSource::new(
                    HpChangeSourceCategories::PhysicalDamage(MeleeOrRanged::Ranged),
                    Some(PhysicalDamageTypes::Piercing),
                    None,
                )],
                1,
                Some(requirements),
                None,
                None,
            )
        }
        TwoHandedRangedWeapons::MilitaryBow => {
            requirements.insert(CombatAttributes::Dexterity, 45);
            requirements.insert(CombatAttributes::Strength, 15);
            WeaponGenerationTemplate::new(
                Range::new(8, 10),
                Range::new(12, 26),
                Some(1),
                vec![HpChangeSource::new(
                    HpChangeSourceCategories::PhysicalDamage(MeleeOrRanged::Ranged),
                    Some(PhysicalDamageTypes::Piercing),
                    None,
                )],
                1,
                Some(requirements),
                None,
                None,
            )
        }
        TwoHandedRangedWeapons::EtherBow => {
            requirements.insert(CombatAttributes::Dexterity, 25);
            requirements.insert(CombatAttributes::Intelligence, 25);

            WeaponGenerationTemplate::new(
                Range::new(7, 10),
                Range::new(10, 22),
                Some(1),
                vec![HpChangeSource::new(
                    HpChangeSourceCategories::MagicalDamage(Evadable::new(true)),
                    Some(PhysicalDamageTypes::Piercing),
                    None,
                )],
                1,
                Some(requirements),
                None,
                None,
            )
        }
    }
}

pub static TWO_HANDED_RANGED_WEAPON_GENERATION_TEMPLATES: Lazy<
    HashMap<TwoHandedRangedWeapons, WeaponGenerationTemplate>,
> = Lazy::new(|| generate_templates(two_handed_ranged_weapon_template_from_base_item));

pub static TWO_HANDED_RANGED_WEAPONS_BY_LEVEL: Lazy<HashMap<u8, Vec<TwoHandedRangedWeapons>>> =
    Lazy::new(|| {
        let items_and_level_ranges: Vec<(&TwoHandedRangedWeapons, &Range<u8>)> =
            TWO_HANDED_RANGED_WEAPON_GENERATION_TEMPLATES
                .iter()
                .collect::<Vec<(&TwoHandedRangedWeapons, &WeaponGenerationTemplate)>>()
                .iter()
                .map(|template| (template.0, &template.1.template_properties.level_range))
                .collect::<Vec<(&TwoHandedRangedWeapons, &Range<u8>)>>();
        items_by_level(items_and_level_ranges)
    });
