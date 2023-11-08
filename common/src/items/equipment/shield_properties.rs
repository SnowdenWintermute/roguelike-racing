use serde::Deserialize;
use serde::Serialize;
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
