use super::magical_elements::MagicalElements;
use core::fmt;
use serde::Deserialize;
use serde::Serialize;
use strum_macros::EnumIter;

#[derive(Serialize, Deserialize, Eq, PartialEq, Clone, Debug, Default)]
pub struct HpChangeSource {
    pub category: HpChangeSourceCategories,
    pub sub_category: Option<HpChangeSourceSubCategories>,
    pub element: Option<MagicalElements>,
}

impl HpChangeSource {
    pub fn new(
        category: HpChangeSourceCategories,
        sub_category: Option<HpChangeSourceSubCategories>,
        element: Option<MagicalElements>,
    ) -> Self {
        HpChangeSource {
            category,
            sub_category,
            element,
        }
    }
}

#[derive(Serialize, Deserialize, Hash, Eq, PartialEq, Copy, Clone, Debug, Default)]
pub struct Evadable(pub bool);
impl Evadable {
    pub fn new(evadable: bool) -> Self {
        Evadable(evadable)
    }
}

#[derive(Serialize, Deserialize, EnumIter, Hash, Eq, PartialEq, Copy, Clone, Debug, Default)]
pub enum HpChangeSourceCategories {
    #[default]
    PhysicalDamage,
    MagicalDamage(Evadable),
    Healing,
    Direct,
}

#[derive(Serialize, Deserialize, Eq, PartialEq, Clone, Debug, Default)]
pub enum HpChangeSourceSubCategories {
    #[default]
    Blunt,
    Slashing,
    Piercing,
}

impl fmt::Display for HpChangeSourceSubCategories {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            HpChangeSourceSubCategories::Slashing => write!(f, "Slashing"),
            HpChangeSourceSubCategories::Blunt => write!(f, "Blunt"),
            HpChangeSourceSubCategories::Piercing => write!(f, "Piercing"),
        }
    }
}

impl fmt::Display for HpChangeSourceCategories {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                HpChangeSourceCategories::PhysicalDamage => "Physical Damage",
                HpChangeSourceCategories::MagicalDamage(Evadable(evadable)) => match evadable {
                    true => "Evadable Magical Damage",
                    false => "Magical Damage",
                },
                HpChangeSourceCategories::Healing => "Healing",
                HpChangeSourceCategories::Direct => "Direct",
            },
        )
    }
}
