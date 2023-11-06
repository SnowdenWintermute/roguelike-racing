pub mod one_handed_melee_weapon_generation_templates;
use super::{
    affixes::{PrefixTypes, SuffixTypes},
    item_generation_template_properties::{
        ItemGenerationTemplate, ItemGenerationTemplateAffixModifiers,
        ItemGenerationTemplateProperties,
    },
};
use crate::{app_consts::DEEPEST_FLOOR, combatants::CombatAttributes, primatives::Range};
use core::fmt;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Serialize, Deserialize, Hash, Eq, PartialEq, Copy, Clone, Debug)]
pub enum DamageCategories {
    Direct(DamageTypes),
    Physical(DamageTypes),
    Magical(DamageTypes),
}

impl Default for DamageCategories {
    fn default() -> DamageCategories {
        DamageCategories::Direct(DamageTypes::Pure)
    }
}

#[derive(Serialize, Deserialize, EnumIter, Hash, Eq, PartialEq, Copy, Clone, Debug, Default)]
pub enum DamageTypes {
    #[default]
    Pure,
    Slashing,
    Blunt,
    Piercing,
    Fire,
    Ice,
    Lightning,
    Water,
    Earth,
    Wind,
    Dark,
    Light,
}

impl fmt::Display for DamageTypes {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DamageTypes::Pure => write!(f, "Pure"),
            DamageTypes::Slashing => write!(f, "Slashing"),
            DamageTypes::Blunt => write!(f, "Blunt"),
            DamageTypes::Piercing => write!(f, "Piercing"),
            DamageTypes::Fire => write!(f, "Fire"),
            DamageTypes::Ice => write!(f, "Ice"),
            DamageTypes::Lightning => write!(f, "Lightning"),
            DamageTypes::Water => write!(f, "Water"),
            DamageTypes::Earth => write!(f, "Earth"),
            DamageTypes::Wind => write!(f, "Wind"),
            DamageTypes::Dark => write!(f, "Dark"),
            DamageTypes::Light => write!(f, "Light"),
        }
    }
}

#[derive(Serialize, Deserialize, Default, EnumIter, Hash, Eq, PartialEq, Copy, Clone, Debug)]
pub enum OneHandedMeleeWeapons {
    #[default]
    // BLUNT
    Stick,
    Mace,
    Morningstar,
    WarHammer,
    // SLASHING
    ShortSword,
    Blade,
    RuneSword,
    BroadSword,
    BastardSword,
    // PIERCING
    Dagger,
    Rapier,
    ShortSpear,
}

impl fmt::Display for OneHandedMeleeWeapons {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            OneHandedMeleeWeapons::Mace => write!(f, "Mace"),
            OneHandedMeleeWeapons::Morningstar => write!(f, "Morning Star"),
            OneHandedMeleeWeapons::WarHammer => write!(f, "War Hammer"),
            OneHandedMeleeWeapons::ShortSword => write!(f, "Short Sword"),
            OneHandedMeleeWeapons::Blade => write!(f, "Blade"),
            OneHandedMeleeWeapons::BroadSword => write!(f, "Broad Sword"),
            OneHandedMeleeWeapons::BastardSword => write!(f, "Bastard Sword"),
            OneHandedMeleeWeapons::Stick => write!(f, "Stick"),
            OneHandedMeleeWeapons::RuneSword => write!(f, "Rune Sword"),
            OneHandedMeleeWeapons::Dagger => write!(f, "Dagger"),
            OneHandedMeleeWeapons::Rapier => write!(f, "Rapier"),
            OneHandedMeleeWeapons::ShortSpear => write!(f, "Short Spear"),
        }
    }
}

pub struct WeaponGenerationTemplate {
    pub damage_classifications: Vec<DamageCategories>,
    pub template_properties: ItemGenerationTemplateProperties,
}

impl ItemGenerationTemplate for WeaponGenerationTemplate {
    fn get_level_range(&self) -> &Range<u8> {
        &self.template_properties.level_range
    }
}

impl WeaponGenerationTemplate {
    pub fn new(
        level_range: Range<u8>,
        damage: Range<u8>,
        max_durability: u8,
        damage_classifications: Vec<DamageCategories>,
        requirements: HashMap<CombatAttributes, u8>,
        affix_modifiers: Option<ItemGenerationTemplateAffixModifiers>,
    ) -> WeaponGenerationTemplate {
        WeaponGenerationTemplate {
            template_properties: ItemGenerationTemplateProperties {
                level_range,
                ac_range: None,
                damage: Some(damage),
                max_durability,
                requirements,
                affix_modifiers,
            },
            damage_classifications,
        }
    }
}
