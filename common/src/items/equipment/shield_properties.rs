use serde::Deserialize;
use serde::Serialize;
use std::fmt::Display;
use strum_macros::EnumIter;

#[derive(Serialize, Deserialize, Eq, PartialEq, Clone, Debug, Default)]
pub struct ShieldProperties {
    pub size: ShieldSizes,
    pub armor_class: u8,
}

impl ShieldProperties {
    pub fn new(size: ShieldSizes, armor_class: u8) -> ShieldProperties {
        ShieldProperties { size, armor_class }
    }
}

#[derive(Debug, EnumIter, Clone, Copy, PartialEq, Serialize, Deserialize, Eq, Default)]
pub enum ShieldSizes {
    #[default]
    Small,
    Medium,
    Large,
}

impl Display for ShieldSizes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ShieldSizes::Small => write!(f, "Small"),
            ShieldSizes::Medium => write!(f, "Medium"),
            ShieldSizes::Large => write!(f, "Large"),
        }
    }
}
