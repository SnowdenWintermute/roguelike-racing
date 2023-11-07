use super::{DamageClassifications, DamageTypes, OneHandedMeleeWeapons, WeaponGenerationTemplate};
use crate::{
    combatants::CombatAttributes, items::items_by_level::items_by_level, primatives::Range,
};
use once_cell::sync::Lazy;
use std::collections::HashMap;
use strum::IntoEnumIterator;

pub static ONE_HANDED_MELEE_WEAPON_GENERATION_TEMPLATES: Lazy<
    HashMap<OneHandedMeleeWeapons, WeaponGenerationTemplate>,
> = Lazy::new(|| {
    let mut m = HashMap::new();
    let items: Vec<OneHandedMeleeWeapons> = OneHandedMeleeWeapons::iter().collect();
    let mut i = 0;
    while i < items.len() {
        let item = items[i];
        let mut requirements: HashMap<CombatAttributes, u8> = HashMap::new();
        let template = match item {
            OneHandedMeleeWeapons::Stick => WeaponGenerationTemplate::new(
                Range::new(1, 3),
                Range::new(1, 3),
                1,
                vec![DamageClassifications::Physical(DamageTypes::Blunt)],
                1,
                requirements,
                None,
                None,
            ),
            OneHandedMeleeWeapons::Mace => WeaponGenerationTemplate::new(
                Range::new(2, 6),
                Range::new(1, 8),
                1,
                vec![DamageClassifications::Physical(DamageTypes::Blunt)],
                1,
                requirements,
                None,
                None,
            ),
            OneHandedMeleeWeapons::Morningstar => WeaponGenerationTemplate::new(
                Range::new(4, 8),
                Range::new(2, 12),
                1,
                vec![DamageClassifications::Physical(DamageTypes::Blunt)],
                1,
                requirements,
                None,
                None,
            ),
            OneHandedMeleeWeapons::WarHammer => WeaponGenerationTemplate::new(
                Range::new(8, 10),
                Range::new(4, 16),
                1,
                vec![DamageClassifications::Physical(DamageTypes::Blunt)],
                1,
                requirements,
                None,
                None,
            ),
            OneHandedMeleeWeapons::ShortSword => WeaponGenerationTemplate::new(
                Range::new(2, 4),
                Range::new(2, 6),
                1,
                vec![DamageClassifications::Physical(DamageTypes::Slashing)],
                1,
                requirements,
                None,
                None,
            ),
            OneHandedMeleeWeapons::Blade => WeaponGenerationTemplate::new(
                Range::new(3, 5),
                Range::new(4, 8),
                1,
                vec![DamageClassifications::Physical(DamageTypes::Slashing)],
                1,
                requirements,
                None,
                None,
            ),
            OneHandedMeleeWeapons::BroadSword => WeaponGenerationTemplate::new(
                Range::new(5, 8),
                Range::new(4, 12),
                1,
                vec![DamageClassifications::Physical(DamageTypes::Slashing)],
                1,
                requirements,
                None,
                None,
            ),
            OneHandedMeleeWeapons::BastardSword => WeaponGenerationTemplate::new(
                Range::new(8, 10),
                Range::new(6, 15),
                1,
                vec![
                    DamageClassifications::Physical(DamageTypes::Slashing),
                    DamageClassifications::Physical(DamageTypes::Piercing),
                ],
                2,
                requirements,
                None,
                None,
            ),
            OneHandedMeleeWeapons::Dagger => WeaponGenerationTemplate::new(
                Range::new(1, 3),
                Range::new(1, 4),
                1,
                vec![DamageClassifications::Physical(DamageTypes::Piercing)],
                1,
                requirements,
                None,
                None,
            ),
            OneHandedMeleeWeapons::Rapier => WeaponGenerationTemplate::new(
                Range::new(3, 7),
                Range::new(1, 11),
                1,
                vec![DamageClassifications::Physical(DamageTypes::Piercing)],
                1,
                requirements,
                None,
                None,
            ),
            OneHandedMeleeWeapons::ShortSpear => WeaponGenerationTemplate::new(
                Range::new(6, 9),
                Range::new(4, 13),
                1,
                vec![DamageClassifications::Physical(DamageTypes::Piercing)],
                1,
                requirements,
                None,
                None,
            ),
            OneHandedMeleeWeapons::RuneSword => WeaponGenerationTemplate::new(
                Range::new(5, 10),
                Range::new(2, 12),
                1,
                vec![
                    DamageClassifications::Physical(DamageTypes::Fire),
                    DamageClassifications::Physical(DamageTypes::Ice),
                    DamageClassifications::Physical(DamageTypes::Water),
                    DamageClassifications::Physical(DamageTypes::Lightning),
                    DamageClassifications::Physical(DamageTypes::Earth),
                    DamageClassifications::Physical(DamageTypes::Wind),
                ],
                1,
                requirements,
                None,
                None,
            ),
            OneHandedMeleeWeapons::EtherBlade => WeaponGenerationTemplate::new(
                Range::new(5, 8),
                Range::new(6, 10),
                1,
                vec![DamageClassifications::Magical(DamageTypes::Slashing)],
                1,
                requirements,
                None,
                None,
            ),
        };

        m.insert(item, template);
        i += 1;
    }
    m
});

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
