use super::WeaponGenerationTemplate;
use crate::combatants::CombatAttributes;
use crate::items::equipment::two_handed_ranged_weapons::TwoHandedRangedWeapons;
use crate::items::equipment::weapon_properties::{DamageClassifications, DamageTypes};
use crate::items::items_by_level::items_by_level;
use crate::primatives::Range;
use once_cell::sync::Lazy;
use std::collections::HashMap;
use strum::IntoEnumIterator;

pub static TWO_HANDED_RANGED_WEAPON_GENERATION_TEMPLATES: Lazy<
    HashMap<TwoHandedRangedWeapons, WeaponGenerationTemplate>,
> = Lazy::new(|| {
    let mut m = HashMap::new();
    let items: Vec<TwoHandedRangedWeapons> = TwoHandedRangedWeapons::iter().collect();
    let mut i = 0;
    while i < items.len() {
        let item = items[i];
        let mut requirements: HashMap<CombatAttributes, u8> = HashMap::new();
        let template = match item {
            TwoHandedRangedWeapons::ShortBow => WeaponGenerationTemplate::new(
                Range::new(1, 4),
                Range::new(2, 7),
                1,
                vec![DamageClassifications::Physical(DamageTypes::Piercing)],
                1,
                requirements,
                None,
                None,
            ),
            TwoHandedRangedWeapons::RecurveBow => WeaponGenerationTemplate::new(
                Range::new(3, 6),
                Range::new(5, 10),
                1,
                vec![DamageClassifications::Physical(DamageTypes::Piercing)],
                1,
                requirements,
                None,
                None,
            ),
            TwoHandedRangedWeapons::CompositeBow => WeaponGenerationTemplate::new(
                Range::new(5, 8),
                Range::new(8, 16),
                1,
                vec![DamageClassifications::Physical(DamageTypes::Piercing)],
                1,
                requirements,
                None,
                None,
            ),
            TwoHandedRangedWeapons::MilitaryBow => WeaponGenerationTemplate::new(
                Range::new(8, 10),
                Range::new(12, 26),
                1,
                vec![DamageClassifications::Physical(DamageTypes::Piercing)],
                1,
                requirements,
                None,
                None,
            ),
            TwoHandedRangedWeapons::EtherBow => WeaponGenerationTemplate::new(
                Range::new(7, 10),
                Range::new(10, 22),
                1,
                vec![DamageClassifications::Magical(DamageTypes::Piercing)],
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
