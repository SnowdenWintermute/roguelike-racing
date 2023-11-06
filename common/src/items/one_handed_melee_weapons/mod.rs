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

pub enum DamageCategories {
    Physical(DamageTypes),
    Magical(DamageTypes),
}

#[derive(Serialize, Deserialize, EnumIter, Hash, Eq, PartialEq, Copy, Clone, Debug, Default)]
pub enum DamageTypes {
    #[default]
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
    Club,
}

impl fmt::Display for OneHandedMeleeWeapons {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            OneHandedMeleeWeapons::Club => todo!(),
        }
    }
}

pub struct WeaponGenerationTemplate {
    pub damage_classification: DamageCategories,
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
        damage_classification: DamageCategories,
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
            damage_classification,
        }
    }
}
