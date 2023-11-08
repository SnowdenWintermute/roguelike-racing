use super::generate_templates::generate_templates;
use super::WeaponGenerationTemplate;
use crate::combatants::CombatAttributes;
use crate::items::equipment::two_handed_melee_weapons::TwoHandedMeleeWeapons;
use crate::items::equipment::weapon_properties::{DamageClassifications, DamageTypes};
use crate::items::items_by_level::items_by_level;
use crate::primatives::Range;
use once_cell::sync::Lazy;
use std::collections::HashMap;
use strum::IntoEnumIterator;

fn two_handed_melee_weapon_template_from_base_item(
    item: &TwoHandedMeleeWeapons,
    mut requirements: HashMap<CombatAttributes, u8>,
) -> WeaponGenerationTemplate {
    match item {
        TwoHandedMeleeWeapons::BoStaff => WeaponGenerationTemplate::new(
            Range::new(1, 4),
            Range::new(2, 8),
            Some(1),
            vec![DamageClassifications::Physical(DamageTypes::Blunt)],
            1,
            requirements,
            None,
            None,
        ),
        TwoHandedMeleeWeapons::Spear => WeaponGenerationTemplate::new(
            Range::new(2, 5),
            Range::new(3, 9),
            Some(1),
            vec![DamageClassifications::Physical(DamageTypes::Piercing)],
            1,
            requirements,
            None,
            None,
        ),
        TwoHandedMeleeWeapons::Bardiche => WeaponGenerationTemplate::new(
            Range::new(2, 5),
            Range::new(5, 11),
            Some(1),
            vec![DamageClassifications::Physical(DamageTypes::Slashing)],
            1,
            requirements,
            None,
            None,
        ),
        TwoHandedMeleeWeapons::SplittingMaul => WeaponGenerationTemplate::new(
            Range::new(3, 6),
            Range::new(6, 12),
            Some(1),
            vec![
                DamageClassifications::Physical(DamageTypes::Blunt),
                DamageClassifications::Physical(DamageTypes::Piercing),
            ],
            2,
            requirements,
            None,
            None,
        ),
        TwoHandedMeleeWeapons::Maul => WeaponGenerationTemplate::new(
            Range::new(5, 7),
            Range::new(9, 14),
            Some(1),
            vec![DamageClassifications::Physical(DamageTypes::Blunt)],
            1,
            requirements,
            None,
            None,
        ),
        TwoHandedMeleeWeapons::BattleAxe => WeaponGenerationTemplate::new(
            Range::new(5, 7),
            Range::new(6, 17),
            Some(1),
            vec![DamageClassifications::Physical(DamageTypes::Slashing)],
            1,
            requirements,
            None,
            None,
        ),
        TwoHandedMeleeWeapons::Glaive => WeaponGenerationTemplate::new(
            Range::new(5, 7),
            Range::new(8, 16),
            Some(1),
            vec![
                DamageClassifications::Physical(DamageTypes::Slashing),
                DamageClassifications::Physical(DamageTypes::Piercing),
            ],
            2,
            requirements,
            None,
            None,
        ),
        TwoHandedMeleeWeapons::ElementalStaff => WeaponGenerationTemplate::new(
            Range::new(6, 9),
            Range::new(10, 18),
            Some(1),
            vec![
                DamageClassifications::Physical(DamageTypes::Fire),
                DamageClassifications::Physical(DamageTypes::Ice),
                DamageClassifications::Physical(DamageTypes::Water),
                DamageClassifications::Physical(DamageTypes::Lightning),
                DamageClassifications::Physical(DamageTypes::Wind),
                DamageClassifications::Physical(DamageTypes::Earth),
            ],
            2,
            requirements,
            None,
            None,
        ),
        TwoHandedMeleeWeapons::Trident => WeaponGenerationTemplate::new(
            Range::new(8, 10),
            Range::new(14, 26),
            Some(1),
            vec![
                DamageClassifications::Magical(DamageTypes::Water),
                DamageClassifications::Physical(DamageTypes::Piercing),
            ],
            2,
            requirements,
            None,
            None,
        ),
        TwoHandedMeleeWeapons::Halberd => WeaponGenerationTemplate::new(
            Range::new(8, 10),
            Range::new(12, 25),
            Some(1),
            vec![
                DamageClassifications::Physical(DamageTypes::Piercing),
                DamageClassifications::Physical(DamageTypes::Slashing),
            ],
            2,
            requirements,
            None,
            None,
        ),
        TwoHandedMeleeWeapons::GreatAxe => WeaponGenerationTemplate::new(
            Range::new(9, 10),
            Range::new(15, 35),
            Some(1),
            vec![DamageClassifications::Physical(DamageTypes::Slashing)],
            1,
            requirements,
            None,
            None,
        ),
        TwoHandedMeleeWeapons::GravityHammer => WeaponGenerationTemplate::new(
            Range::new(9, 10),
            Range::new(20, 30),
            Some(1),
            vec![DamageClassifications::Physical(DamageTypes::Blunt)],
            1,
            requirements,
            None,
            None,
        ),
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
