pub mod body_armor_generation_templates;
pub mod body_armor_possible_affixes;
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

#[derive(Serialize, Deserialize, EnumIter, Hash, Eq, PartialEq, Copy, Clone, Debug, Default)]
pub enum ArmorCategories {
    #[default]
    Cloth,
    Leather,
    Mail,
    Plate,
}

impl fmt::Display for ArmorCategories {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ArmorCategories::Cloth => write!(f, "Cloth"),
            ArmorCategories::Leather => write!(f, "Leather"),
            ArmorCategories::Mail => write!(f, "Mail"),
            ArmorCategories::Plate => write!(f, "Plate"),
        }
    }
}

#[derive(Serialize, Deserialize, Default, EnumIter, Hash, Eq, PartialEq, Copy, Clone, Debug)]
pub enum BodyArmors {
    #[default]
    Rags,
    Cape,
    Cloak,
    Robe,
    Kevlar,
    LeatherArmor,
    HardLeatherArmor,
    StuddedLeatherArmor,
    DemonsaurArmor,
    RingMail,
    ChainMail,
    ScaleMail,
    SplintMail,
    OhmushellMail,
    BreastPlate,
    FieldPlate,
    GothicPlate,
    FullPlate,
    ShardPlate,
}

impl fmt::Display for BodyArmors {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            BodyArmors::Rags => write!(f, "Rags"),
            BodyArmors::Cape => write!(f, "Cape"),
            BodyArmors::Cloak => write!(f, "Cloak"),
            BodyArmors::Robe => write!(f, "Robe"),
            BodyArmors::Kevlar => write!(f, "Kevlar"),
            BodyArmors::LeatherArmor => write!(f, "Leather Armor"),
            BodyArmors::HardLeatherArmor => write!(f, "Hard Leather Armor"),
            BodyArmors::StuddedLeatherArmor => write!(f, "Studded Leather Armor"),
            BodyArmors::DemonsaurArmor => write!(f, "Demonsaur Armor"),
            BodyArmors::RingMail => write!(f, "Ring Mail"),
            BodyArmors::ChainMail => write!(f, "Chain Mail"),
            BodyArmors::ScaleMail => write!(f, "Scale Mail"),
            BodyArmors::SplintMail => write!(f, "Splint Mail"),
            BodyArmors::OhmushellMail => write!(f, "Ohmushell Mail"),
            BodyArmors::BreastPlate => write!(f, "Breast Plate"),
            BodyArmors::FieldPlate => write!(f, "Field Plate"),
            BodyArmors::GothicPlate => write!(f, "Gothic Plate"),
            BodyArmors::FullPlate => write!(f, "Full Plate"),
            BodyArmors::ShardPlate => write!(f, "Shard Plate"),
        }
    }
}

pub struct ArmorGenerationTemplate {
    pub category: ArmorCategories,
    pub template_properties: ItemGenerationTemplateProperties,
}

impl ItemGenerationTemplate for ArmorGenerationTemplate {
    fn get_level_range(&self) -> &Range<u8> {
        &self.template_properties.level_range
    }
}

impl ArmorGenerationTemplate {
    pub fn new(
        level_range: Range<u8>,
        ac_range: Range<u8>,
        max_durability: u8,
        category: ArmorCategories,
        requirements: HashMap<CombatAttributes, u8>,
        affix_modifiers: Option<ItemGenerationTemplateAffixModifiers>,
    ) -> ArmorGenerationTemplate {
        ArmorGenerationTemplate {
            template_properties: ItemGenerationTemplateProperties {
                level_range,
                ac_range: Some(ac_range),
                damage: None,
                max_durability,
                requirements,
                affix_modifiers,
            },
            category,
        }
    }
}
