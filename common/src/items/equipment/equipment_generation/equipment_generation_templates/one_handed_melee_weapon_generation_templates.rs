use super::generate_templates::generate_templates;
use super::WeaponGenerationTemplate;
use crate::combatants::combat_attributes::CombatAttributes;
use crate::items::equipment::one_handed_melee_weapons::OneHandedMeleeWeapons;
use crate::items::equipment::weapon_properties::DamageClassifications;
use crate::items::equipment::weapon_properties::DamageTypes;
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
            vec![DamageClassifications::Physical(DamageTypes::Blunt)],
            1,
            Some(requirements),
            None,
            None,
        ),
        OneHandedMeleeWeapons::Mace => WeaponGenerationTemplate::new(
            Range::new(2, 6),
            Range::new(1, 8),
            Some(1),
            vec![DamageClassifications::Physical(DamageTypes::Blunt)],
            1,
            Some(requirements),
            None,
            None,
        ),
        OneHandedMeleeWeapons::Morningstar => WeaponGenerationTemplate::new(
            Range::new(4, 8),
            Range::new(2, 12),
            Some(1),
            vec![DamageClassifications::Physical(DamageTypes::Blunt)],
            1,
            Some(requirements),
            None,
            None,
        ),
        OneHandedMeleeWeapons::WarHammer => {
            requirements.insert(CombatAttributes::Strength, 20);
            WeaponGenerationTemplate::new(
                Range::new(8, 10),
                Range::new(4, 16),
                Some(1),
                vec![DamageClassifications::Physical(DamageTypes::Blunt)],
                1,
                Some(requirements),
                None,
                None,
            )
        }
        OneHandedMeleeWeapons::ShortSword => WeaponGenerationTemplate::new(
            Range::new(2, 4),
            Range::new(2, 6),
            Some(1),
            vec![DamageClassifications::Physical(DamageTypes::Slashing)],
            1,
            Some(requirements),
            None,
            None,
        ),
        OneHandedMeleeWeapons::Blade => WeaponGenerationTemplate::new(
            Range::new(3, 5),
            Range::new(4, 8),
            Some(1),
            vec![DamageClassifications::Physical(DamageTypes::Slashing)],
            1,
            Some(requirements),
            None,
            None,
        ),
        OneHandedMeleeWeapons::BroadSword => WeaponGenerationTemplate::new(
            Range::new(5, 8),
            Range::new(4, 12),
            Some(1),
            vec![DamageClassifications::Physical(DamageTypes::Slashing)],
            1,
            Some(requirements),
            None,
            None,
        ),
        OneHandedMeleeWeapons::BastardSword => WeaponGenerationTemplate::new(
            Range::new(8, 10),
            Range::new(6, 15),
            Some(1),
            vec![
                DamageClassifications::Physical(DamageTypes::Slashing),
                DamageClassifications::Physical(DamageTypes::Piercing),
            ],
            2,
            Some(requirements),
            None,
            None,
        ),
        OneHandedMeleeWeapons::Dagger => WeaponGenerationTemplate::new(
            Range::new(1, 3),
            Range::new(1, 4),
            Some(1),
            vec![DamageClassifications::Physical(DamageTypes::Piercing)],
            1,
            Some(requirements),
            None,
            None,
        ),
        OneHandedMeleeWeapons::Rapier => WeaponGenerationTemplate::new(
            Range::new(3, 7),
            Range::new(1, 11),
            Some(1),
            vec![DamageClassifications::Physical(DamageTypes::Piercing)],
            1,
            Some(requirements),
            None,
            None,
        ),
        OneHandedMeleeWeapons::ShortSpear => WeaponGenerationTemplate::new(
            Range::new(6, 9),
            Range::new(4, 13),
            Some(1),
            vec![DamageClassifications::Physical(DamageTypes::Piercing)],
            1,
            Some(requirements),
            None,
            None,
        ),
        OneHandedMeleeWeapons::RuneSword => WeaponGenerationTemplate::new(
            Range::new(5, 10),
            Range::new(2, 12),
            Some(1),
            vec![
                DamageClassifications::Physical(DamageTypes::Fire),
                DamageClassifications::Physical(DamageTypes::Ice),
                DamageClassifications::Physical(DamageTypes::Water),
                DamageClassifications::Physical(DamageTypes::Lightning),
                DamageClassifications::Physical(DamageTypes::Earth),
                DamageClassifications::Physical(DamageTypes::Wind),
            ],
            1,
            Some(requirements),
            None,
            None,
        ),
        OneHandedMeleeWeapons::EtherBlade => WeaponGenerationTemplate::new(
            Range::new(5, 8),
            Range::new(6, 10),
            Some(1),
            vec![DamageClassifications::Magical(DamageTypes::Slashing)],
            1,
            Some(requirements),
            None,
            None,
        ),
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
