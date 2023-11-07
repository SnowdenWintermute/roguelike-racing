use core::fmt;
use serde::{Deserialize, Serialize};
use strum_macros::EnumIter;

#[derive(Serialize, Deserialize, Eq, PartialEq, Clone, Debug, Default)]
pub struct ArmorProperties {
    pub armor_category: ArmorCategories,
    pub armor_class: u8,
}

impl ArmorProperties {
    pub fn new(armor_category: ArmorCategories, armor_class: u8) -> ArmorProperties {
        ArmorProperties {
            armor_category,
            armor_class,
        }
    }
}

#[derive(Serialize, Deserialize, EnumIter, Eq, PartialEq, Copy, Clone, Debug, Default)]
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
