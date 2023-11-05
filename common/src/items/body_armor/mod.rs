pub mod body_armor_generation_templates;
pub mod body_armor_possible_affixes;
pub mod body_armors_by_level;
use super::affixes::{PrefixTypes, SuffixTypes};
use crate::{app_consts::DEEPEST_FLOOR, combatants::CombatAttributes, primatives::Range};
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

pub struct ArmorGenerationTemplate {
    pub level_range: Range<u8>,
    pub category: ArmorCategories,
    pub max_durability: u8,
    pub requirements: HashMap<CombatAttributes, u16>,
}

impl ArmorGenerationTemplate {
    pub fn new(
        min_level: u8,
        max_level: u8,
        max_durability: u8,
        category: ArmorCategories,
        requirements: HashMap<CombatAttributes, u16>,
    ) -> ArmorGenerationTemplate {
        ArmorGenerationTemplate {
            level_range: Range::new(min_level, max_level),
            max_durability,
            category,
            requirements,
        }
    }
}
