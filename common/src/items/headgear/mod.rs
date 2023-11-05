mod headgear_generation_templates;
pub mod headgear_possible_affixes;
use super::affixes::{PrefixTypes, SuffixTypes};
use crate::{app_consts::DEEPEST_FLOOR, combatants::CombatAttributes, primatives::Range};
use core::fmt;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Serialize, Deserialize, Default, EnumIter, Hash, Eq, PartialEq, Copy, Clone, Debug)]
pub enum HeadGears {
    #[default]
    // CLOTH
    Cap,
    Bandana,
    PaddedCap,
    Ribbon,
    WizardHat,
    // LEATHER
    Eyepatch,
    LeatherHat,
    LeatherHelm,
    DemonsaurHelm,
    // MAIL
    Hairpin,
    Skullcap,
    Coif,
    OhmushellMask,
    // PLATE
    Circlet,
    Crown,
    FullHelm,
    GreatHelm,
}

impl fmt::Display for HeadGears {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            _ => write!(f, "unhandled headgear display"),
        }
    }
}
