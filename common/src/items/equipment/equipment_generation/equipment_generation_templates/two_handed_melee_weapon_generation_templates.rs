use super::generate_templates::generate_templates;
use super::WeaponGenerationTemplate;
use crate::combat::hp_change_source_types::Evadable;
use crate::combat::hp_change_source_types::HpChangeSource;
use crate::combat::hp_change_source_types::HpChangeSourceCategories;
use crate::combat::hp_change_source_types::MeleeOrRanged;
use crate::combat::hp_change_source_types::PhysicalDamageTypes;
use crate::combat::magical_elements::MagicalElements;
use crate::combatants::combat_attributes::CombatAttributes;
use crate::items::equipment::two_handed_melee_weapons::TwoHandedMeleeWeapons;
use crate::items::items_by_level::items_by_level;
use crate::primatives::Range;
use once_cell::sync::Lazy;
use std::collections::HashMap;

fn two_handed_melee_weapon_template_from_base_item(
    item: &TwoHandedMeleeWeapons,
    mut requirements: HashMap<CombatAttributes, u8>,
) -> WeaponGenerationTemplate {
    match item {
        TwoHandedMeleeWeapons::BoStaff => WeaponGenerationTemplate::new(
            Range::new(1, 4),
            Range::new(2, 8),
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
        TwoHandedMeleeWeapons::Spear => {
            requirements.insert(CombatAttributes::Dexterity, 10);
            requirements.insert(CombatAttributes::Strength, 5);
            WeaponGenerationTemplate::new(
                Range::new(2, 5),
                Range::new(3, 9),
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
        TwoHandedMeleeWeapons::Bardiche => {
            requirements.insert(CombatAttributes::Strength, 15);
            WeaponGenerationTemplate::new(
                Range::new(2, 5),
                Range::new(5, 11),
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
        TwoHandedMeleeWeapons::SplittingMaul => {
            requirements.insert(CombatAttributes::Strength, 25);
            requirements.insert(CombatAttributes::Dexterity, 7);
            WeaponGenerationTemplate::new(
                Range::new(3, 6),
                Range::new(6, 12),
                Some(1),
                vec![
                    HpChangeSource::new(
                        HpChangeSourceCategories::PhysicalDamage(MeleeOrRanged::Melee),
                        Some(PhysicalDamageTypes::Blunt),
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
        TwoHandedMeleeWeapons::Maul => {
            requirements.insert(CombatAttributes::Strength, 28);
            WeaponGenerationTemplate::new(
                Range::new(5, 7),
                Range::new(9, 14),
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
        TwoHandedMeleeWeapons::BattleAxe => {
            requirements.insert(CombatAttributes::Strength, 35);
            WeaponGenerationTemplate::new(
                Range::new(5, 7),
                Range::new(6, 17),
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
        TwoHandedMeleeWeapons::Glaive => {
            requirements.insert(CombatAttributes::Strength, 14);
            requirements.insert(CombatAttributes::Dexterity, 35);
            WeaponGenerationTemplate::new(
                Range::new(5, 7),
                Range::new(8, 16),
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
        TwoHandedMeleeWeapons::ElementalStaff => {
            requirements.insert(CombatAttributes::Strength, 20);
            requirements.insert(CombatAttributes::Intelligence, 20);

            WeaponGenerationTemplate::new(
                Range::new(6, 9),
                Range::new(10, 18),
                Some(1),
                vec![
                    HpChangeSource::new(
                        HpChangeSourceCategories::PhysicalDamage(MeleeOrRanged::Melee),
                        Some(PhysicalDamageTypes::Blunt),
                        Some(MagicalElements::Fire),
                    ),
                    HpChangeSource::new(
                        HpChangeSourceCategories::PhysicalDamage(MeleeOrRanged::Melee),
                        Some(PhysicalDamageTypes::Blunt),
                        Some(MagicalElements::Ice),
                    ),
                    HpChangeSource::new(
                        HpChangeSourceCategories::PhysicalDamage(MeleeOrRanged::Melee),
                        Some(PhysicalDamageTypes::Blunt),
                        Some(MagicalElements::Lightning),
                    ),
                    HpChangeSource::new(
                        HpChangeSourceCategories::PhysicalDamage(MeleeOrRanged::Melee),
                        Some(PhysicalDamageTypes::Blunt),
                        Some(MagicalElements::Water),
                    ),
                    HpChangeSource::new(
                        HpChangeSourceCategories::PhysicalDamage(MeleeOrRanged::Melee),
                        Some(PhysicalDamageTypes::Blunt),
                        Some(MagicalElements::Wind),
                    ),
                    HpChangeSource::new(
                        HpChangeSourceCategories::PhysicalDamage(MeleeOrRanged::Melee),
                        Some(PhysicalDamageTypes::Blunt),
                        Some(MagicalElements::Earth),
                    ),
                ],
                2,
                Some(requirements),
                None,
                None,
            )
        }
        TwoHandedMeleeWeapons::Trident => {
            requirements.insert(CombatAttributes::Strength, 15);
            requirements.insert(CombatAttributes::Intelligence, 35);

            WeaponGenerationTemplate::new(
                Range::new(8, 10),
                Range::new(14, 26),
                Some(1),
                vec![
                    HpChangeSource::new(
                        HpChangeSourceCategories::MagicalDamage(Evadable::new(false)),
                        None,
                        Some(MagicalElements::Water),
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
        TwoHandedMeleeWeapons::Halberd => {
            requirements.insert(CombatAttributes::Dexterity, 35);
            requirements.insert(CombatAttributes::Strength, 20);

            WeaponGenerationTemplate::new(
                Range::new(8, 10),
                Range::new(12, 25),
                Some(1),
                vec![
                    HpChangeSource::new(
                        HpChangeSourceCategories::PhysicalDamage(MeleeOrRanged::Melee),
                        Some(PhysicalDamageTypes::Piercing),
                        None,
                    ),
                    HpChangeSource::new(
                        HpChangeSourceCategories::PhysicalDamage(MeleeOrRanged::Melee),
                        Some(PhysicalDamageTypes::Slashing),
                        None,
                    ),
                ],
                2,
                Some(requirements),
                None,
                None,
            )
        }
        TwoHandedMeleeWeapons::GreatAxe => {
            requirements.insert(CombatAttributes::Strength, 65);
            WeaponGenerationTemplate::new(
                Range::new(9, 10),
                Range::new(15, 35),
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
        TwoHandedMeleeWeapons::GravityHammer => {
            requirements.insert(CombatAttributes::Strength, 70);
            WeaponGenerationTemplate::new(
                Range::new(9, 10),
                Range::new(20, 30),
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
        TwoHandedMeleeWeapons::AshStaff => {
            requirements.insert(CombatAttributes::Intelligence, 15);
            WeaponGenerationTemplate::new(
                Range::new(1, 4),
                Range::new(2, 8),
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
        TwoHandedMeleeWeapons::ElmStaff => {
            requirements.insert(CombatAttributes::Intelligence, 25);
            WeaponGenerationTemplate::new(
                Range::new(3, 6),
                Range::new(4, 12),
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
        TwoHandedMeleeWeapons::MahoganyStaff => {
            requirements.insert(CombatAttributes::Intelligence, 45);
            WeaponGenerationTemplate::new(
                Range::new(5, 8),
                Range::new(8, 22),
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
        TwoHandedMeleeWeapons::EbonyStaff => {
            requirements.insert(CombatAttributes::Intelligence, 70);
            WeaponGenerationTemplate::new(
                Range::new(8, 10),
                Range::new(10, 32),
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
    }
}
pub static TWO_HANDED_MELEE_WEAPON_GENERATION_TEMPLATES: Lazy<
    HashMap<TwoHandedMeleeWeapons, WeaponGenerationTemplate>,
> = Lazy::new(|| generate_templates(two_handed_melee_weapon_template_from_base_item));

pub static TWO_HANDED_MELEE_WEAPONS_BY_LEVEL: Lazy<HashMap<u8, Vec<TwoHandedMeleeWeapons>>> =
    Lazy::new(|| {
        let items_and_level_ranges: Vec<(&TwoHandedMeleeWeapons, &Range<u8>)> =
            TWO_HANDED_MELEE_WEAPON_GENERATION_TEMPLATES
                .iter()
                .collect::<Vec<(&TwoHandedMeleeWeapons, &WeaponGenerationTemplate)>>()
                .iter()
                .map(|template| (template.0, &template.1.template_properties.level_range))
                .collect::<Vec<(&TwoHandedMeleeWeapons, &Range<u8>)>>();
        items_by_level(items_and_level_ranges)
    });
