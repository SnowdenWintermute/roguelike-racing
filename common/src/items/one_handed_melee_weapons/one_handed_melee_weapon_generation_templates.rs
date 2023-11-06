use super::{DamageCategories, DamageTypes, OneHandedMeleeWeapons, WeaponGenerationTemplate};
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
                Range::new(1, 1),
                Range::new(1, 1),
                1,
                vec![DamageCategories::Physical(DamageTypes::Blunt)],
                requirements,
                None,
            ),
            OneHandedMeleeWeapons::Mace => todo!(),
            OneHandedMeleeWeapons::Morningstar => todo!(),
            OneHandedMeleeWeapons::WarHammer => todo!(),
            OneHandedMeleeWeapons::ShortSword => todo!(),
            OneHandedMeleeWeapons::Blade => todo!(),
            OneHandedMeleeWeapons::RuneSword => todo!(),
            OneHandedMeleeWeapons::BroadSword => todo!(),
            OneHandedMeleeWeapons::BastardSword => todo!(),
            OneHandedMeleeWeapons::Dagger => todo!(),
            OneHandedMeleeWeapons::Rapier => todo!(),
            OneHandedMeleeWeapons::ShortSpear => todo!(),
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
