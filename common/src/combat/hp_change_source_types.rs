use super::magical_elements::MagicalElements;
use core::fmt;
use serde::Deserialize;
use serde::Serialize;
use strum_macros::EnumIter;

#[derive(Serialize, Deserialize, Eq, PartialEq, Clone, Debug, Default)]
pub struct HpChangeSource {
    pub category: HpChangeSourceCategories,
    pub sub_category: Option<PhysicalDamageTypes>,
    pub element: Option<MagicalElements>,
}

impl HpChangeSource {
    pub fn new(
        category: HpChangeSourceCategories,
        sub_category: Option<PhysicalDamageTypes>,
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
    PhysicalDamage(MeleeOrRanged),
    MagicalDamage(Evadable),
    Healing,
    #[default]
    Direct,
}

#[derive(Serialize, Deserialize, EnumIter, Hash, Eq, PartialEq, Copy, Clone, Debug, Default)]
pub enum MeleeOrRanged {
    #[default]
    Melee,
    Ranged,
}

#[derive(Serialize, Deserialize, Eq, PartialEq, Clone, Debug, Default, Hash)]
pub enum PhysicalDamageTypes {
    #[default]
    Blunt,
    Slashing,
    Piercing,
}

impl fmt::Display for PhysicalDamageTypes {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            PhysicalDamageTypes::Slashing => write!(f, "Slashing"),
            PhysicalDamageTypes::Blunt => write!(f, "Blunt"),
            PhysicalDamageTypes::Piercing => write!(f, "Piercing"),
        }
    }
}

impl fmt::Display for HpChangeSourceCategories {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                HpChangeSourceCategories::PhysicalDamage(melee_or_ranged) =>
                    match melee_or_ranged {
                        MeleeOrRanged::Melee => "Melee",
                        MeleeOrRanged::Ranged => "Ranged",
                    },
                HpChangeSourceCategories::MagicalDamage(Evadable(evadable)) => match evadable {
                    true => "Magical (Evadable)",
                    false => "Magical",
                },
                HpChangeSourceCategories::Healing => "Healing",
                HpChangeSourceCategories::Direct => "Direct",
            },
        )
    }
}
