use super::generate_templates::generate_templates;
use super::WeaponGenerationTemplate;
use crate::combat::hp_change_source_types::Evadable;
use crate::combat::hp_change_source_types::HpChangeSource;
use crate::combat::hp_change_source_types::HpChangeSourceCategories;
use crate::combat::hp_change_source_types::MeleeOrRanged;
use crate::combat::hp_change_source_types::PhysicalDamageTypes;
use crate::combat::magical_elements::MagicalElements;
use crate::combatants::combat_attributes::CombatAttributes;
use crate::items::equipment::one_handed_melee_weapons::OneHandedMeleeWeapons;
use crate::items::items_by_level::items_by_level;
use crate::primatives::Range;
use once_cell::sync::Lazy;
use std::collections::HashMap;

pub fn one_handed_melee_weapon_template_from_base_item(
    item: &OneHandedMeleeWeapons,
    mut requirements: HashMap<CombatAttributes, u8>,
) -> WeaponGenerationTemplate {
    match item {
        OneHandedMeleeWeapons::Stick => WeaponGenerationTemplate::new(
            Range::new(1, 3),
            Range::new(1, 3),
            Some(1),
            vec![HpChangeSource::new(
                HpChangeSourceCategories::PhysicalDamage(MeleeOrRanged::Melee),
                Some(PhysicalDamageTypes::Blunt),
                None,
            )],
            1,
            Some(requirements),
            None,
            None,
        ),
        OneHandedMeleeWeapons::Mace => {
            requirements.insert(CombatAttributes::Strength, 18);
            WeaponGenerationTemplate::new(
                Range::new(2, 6),
                Range::new(1, 8),
                Some(1),
                vec![HpChangeSource::new(
                    HpChangeSourceCategories::PhysicalDamage(MeleeOrRanged::Melee),
                    Some(PhysicalDamageTypes::Blunt),
                    None,
                )],
                1,
                Some(requirements),
                None,
                None,
            )
        }
        OneHandedMeleeWeapons::Morningstar => {
            requirements.insert(CombatAttributes::Strength, 25);
            WeaponGenerationTemplate::new(
                Range::new(4, 8),
                Range::new(2, 12),
                Some(1),
                vec![HpChangeSource::new(
                    HpChangeSourceCategories::PhysicalDamage(MeleeOrRanged::Melee),
                    Some(PhysicalDamageTypes::Blunt),
                    None,
                )],
                1,
                Some(requirements),
                None,
                None,
            )
        }
        OneHandedMeleeWeapons::WarHammer => {
            requirements.insert(CombatAttributes::Strength, 55);
            WeaponGenerationTemplate::new(
                Range::new(8, 10),
                Range::new(4, 16),
                Some(1),
                vec![HpChangeSource::new(
                    HpChangeSourceCategories::PhysicalDamage(MeleeOrRanged::Melee),
                    Some(PhysicalDamageTypes::Blunt),
                    None,
                )],
                1,
                Some(requirements),
                None,
                None,
            )
        }
        OneHandedMeleeWeapons::ShortSword => {
            requirements.insert(CombatAttributes::Strength, 15);
            WeaponGenerationTemplate::new(
                Range::new(2, 4),
                Range::new(2, 6),
                Some(1),
                vec![HpChangeSource::new(
                    HpChangeSourceCategories::PhysicalDamage(MeleeOrRanged::Melee),
                    Some(PhysicalDamageTypes::Slashing),
                    None,
                )],
                1,
                Some(requirements),
                None,
                None,
            )
        }
        OneHandedMeleeWeapons::IceBlade => {
            requirements.insert(CombatAttributes::Strength, 15);
            WeaponGenerationTemplate::new(
                Range::new(2, 4),
                Range::new(2, 6),
                Some(1),
                vec![HpChangeSource::new(
                    HpChangeSourceCategories::PhysicalDamage(MeleeOrRanged::Melee),
                    Some(PhysicalDamageTypes::Slashing),
                    Some(MagicalElements::Ice),
                )],
                1,
                Some(requirements),
                None,
                None,
            )
        }
        OneHandedMeleeWeapons::Blade => {
            requirements.insert(CombatAttributes::Strength, 20);
            requirements.insert(CombatAttributes::Dexterity, 20);
            WeaponGenerationTemplate::new(
                Range::new(3, 5),
                Range::new(4, 8),
                Some(1),
                vec![HpChangeSource::new(
                    HpChangeSourceCategories::PhysicalDamage(MeleeOrRanged::Melee),
                    Some(PhysicalDamageTypes::Slashing),
                    None,
                )],
                1,
                Some(requirements),
                None,
                None,
            )
        }
        OneHandedMeleeWeapons::BroadSword => {
            requirements.insert(CombatAttributes::Strength, 38);
            WeaponGenerationTemplate::new(
                Range::new(5, 8),
                Range::new(4, 12),
                Some(1),
                vec![HpChangeSource::new(
                    HpChangeSourceCategories::PhysicalDamage(MeleeOrRanged::Melee),
                    Some(PhysicalDamageTypes::Slashing),
                    None,
                )],
                1,
                Some(requirements),
                None,
                None,
            )
        }
        OneHandedMeleeWeapons::BastardSword => {
            requirements.insert(CombatAttributes::Strength, 55);
            WeaponGenerationTemplate::new(
                Range::new(8, 10),
                Range::new(6, 15),
                Some(1),
                vec![
                    HpChangeSource::new(
                        HpChangeSourceCategories::PhysicalDamage(MeleeOrRanged::Melee),
                        Some(PhysicalDamageTypes::Slashing),
                        None,
                    ),
                    HpChangeSource::new(
                        HpChangeSourceCategories::PhysicalDamage(MeleeOrRanged::Melee),
                        Some(PhysicalDamageTypes::Piercing),
                        None,
                    ),
                ],
                2,
                Some(requirements),
                None,
                None,
            )
        }
        OneHandedMeleeWeapons::Dagger => WeaponGenerationTemplate::new(
            Range::new(1, 3),
            Range::new(1, 4),
            Some(1),
            vec![HpChangeSource::new(
                HpChangeSourceCategories::PhysicalDamage(MeleeOrRanged::Melee),
                Some(PhysicalDamageTypes::Piercing),
                None,
            )],
            1,
            Some(requirements),
            None,
            None,
        ),
        OneHandedMeleeWeapons::Rapier => {
            requirements.insert(CombatAttributes::Dexterity, 15);
            WeaponGenerationTemplate::new(
                Range::new(3, 7),
                Range::new(1, 11),
                Some(1),
                vec![HpChangeSource::new(
                    HpChangeSourceCategories::PhysicalDamage(MeleeOrRanged::Melee),
                    Some(PhysicalDamageTypes::Piercing),
                    None,
                )],
                1,
                Some(requirements),
                None,
                None,
            )
        }
        OneHandedMeleeWeapons::ShortSpear => {
            requirements.insert(CombatAttributes::Strength, 25);
            requirements.insert(CombatAttributes::Dexterity, 15);
            WeaponGenerationTemplate::new(
                Range::new(6, 9),
                Range::new(4, 13),
                Some(1),
                vec![HpChangeSource::new(
                    HpChangeSourceCategories::PhysicalDamage(MeleeOrRanged::Melee),
                    Some(PhysicalDamageTypes::Piercing),
                    None,
                )],
                1,
                Some(requirements),
                None,
                None,
            )
        }
        OneHandedMeleeWeapons::RuneSword => {
            requirements.insert(CombatAttributes::Strength, 18);
            requirements.insert(CombatAttributes::Intelligence, 15);
            requirements.insert(CombatAttributes::Dexterity, 12);

            WeaponGenerationTemplate::new(
                Range::new(5, 10),
                Range::new(2, 12),
                Some(1),
                vec![
                    HpChangeSource::new(
                        HpChangeSourceCategories::PhysicalDamage(MeleeOrRanged::Melee),
                        Some(PhysicalDamageTypes::Slashing),
                        Some(MagicalElements::Fire),
                    ),
                    HpChangeSource::new(
                        HpChangeSourceCategories::PhysicalDamage(MeleeOrRanged::Melee),
                        Some(PhysicalDamageTypes::Slashing),
                        Some(MagicalElements::Ice),
                    ),
                    HpChangeSource::new(
                        HpChangeSourceCategories::PhysicalDamage(MeleeOrRanged::Melee),
                        Some(PhysicalDamageTypes::Slashing),
                        Some(MagicalElements::Lightning),
                    ),
                    HpChangeSource::new(
                        HpChangeSourceCategories::PhysicalDamage(MeleeOrRanged::Melee),
                        Some(PhysicalDamageTypes::Slashing),
                        Some(MagicalElements::Water),
                    ),
                    HpChangeSource::new(
                        HpChangeSourceCategories::PhysicalDamage(MeleeOrRanged::Melee),
                        Some(PhysicalDamageTypes::Slashing),
                        Some(MagicalElements::Wind),
                    ),
                    HpChangeSource::new(
                        HpChangeSourceCategories::PhysicalDamage(MeleeOrRanged::Melee),
                        Some(PhysicalDamageTypes::Slashing),
                        Some(MagicalElements::Earth),
                    ),
                ],
                1,
                Some(requirements),
                None,
                None,
            )
        }
        OneHandedMeleeWeapons::EtherBlade => {
            requirements.insert(CombatAttributes::Strength, 20);
            requirements.insert(CombatAttributes::Intelligence, 15);

            WeaponGenerationTemplate::new(
                Range::new(5, 8),
                Range::new(6, 10),
                Some(1),
                vec![HpChangeSource::new(
                    HpChangeSourceCategories::MagicalDamage(Evadable::new(true)),
                    Some(PhysicalDamageTypes::Slashing),
                    None,
                )],
                1,
                Some(requirements),
                None,
                None,
            )
        }
        OneHandedMeleeWeapons::MapleWand => {
            requirements.insert(CombatAttributes::Intelligence, 7);
            WeaponGenerationTemplate::new(
                Range::new(2, 6),
                Range::new(1, 8),
                Some(1),
                vec![HpChangeSource::new(
                    HpChangeSourceCategories::MagicalDamage(Evadable(true)),
                    None,
                    None,
                )],
                1,
                Some(requirements),
                None,
                None,
            )
        }
        OneHandedMeleeWeapons::WillowWand => {
            requirements.insert(CombatAttributes::Intelligence, 20);
            WeaponGenerationTemplate::new(
                Range::new(5, 7),
                Range::new(2, 10),
                Some(1),
                vec![HpChangeSource::new(
                    HpChangeSourceCategories::MagicalDamage(Evadable(true)),
                    None,
                    None,
                )],
                1,
                Some(requirements),
                None,
                None,
            )
        }
        OneHandedMeleeWeapons::YewWand => {
            requirements.insert(CombatAttributes::Intelligence, 35);
            WeaponGenerationTemplate::new(
                Range::new(5, 7),
                Range::new(3, 13),
                Some(1),
                vec![HpChangeSource::new(
                    HpChangeSourceCategories::MagicalDamage(Evadable(true)),
                    None,
                    None,
                )],
                1,
                Some(requirements),
                None,
                None,
            )
        }
        OneHandedMeleeWeapons::RoseWand => {
            requirements.insert(CombatAttributes::Intelligence, 55);
            WeaponGenerationTemplate::new(
                Range::new(5, 7),
                Range::new(6, 15),
                Some(1),
                vec![HpChangeSource::new(
                    HpChangeSourceCategories::MagicalDamage(Evadable(true)),
                    None,
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

pub static ONE_HANDED_MELEE_WEAPON_GENERATION_TEMPLATES: Lazy<
    HashMap<OneHandedMeleeWeapons, WeaponGenerationTemplate>,
> = Lazy::new(|| generate_templates(one_handed_melee_weapon_template_from_base_item));

pub static ONE_HANDED_MELEE_WEAPONS_BY_LEVEL: Lazy<HashMap<u8, Vec<OneHandedMeleeWeapons>>> =
    Lazy::new(|| {
        let items_and_level_ranges: Vec<(&OneHandedMeleeWeapons, &Range<u8>)> =
            ONE_HANDED_MELEE_WEAPON_GENERATION_TEMPLATES
                .iter()
                .collect::<Vec<(&OneHandedMeleeWeapons, &WeaponGenerationTemplate)>>()
                .iter()
                .map(|template| (template.0, &template.1.template_properties.level_range))
                .collect::<Vec<(&OneHandedMeleeWeapons, &Range<u8>)>>();
        items_by_level(items_and_level_ranges)
    });
