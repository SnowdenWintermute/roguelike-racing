use crate::primatives::Range;
use core::fmt;
use serde::Deserialize;
use serde::Serialize;
use std::fmt::Display;
use strum_macros::EnumIter;

#[derive(Serialize, Deserialize, Eq, PartialEq, Clone, Debug, Default)]
pub struct WeaponProperties {
    pub damage_classifications: Vec<DamageClassifications>,
    pub damage: Range<u8>,
}

impl WeaponProperties {
    pub fn new(
        damage_classifications: Vec<DamageClassifications>,
        damage: Range<u8>,
    ) -> WeaponProperties {
        WeaponProperties {
            damage_classifications,
            damage,
        }
    }
}

#[derive(Serialize, Deserialize, Hash, Eq, PartialEq, Copy, Clone, Debug)]
pub enum DamageClassifications {
    Direct(DamageTypes),
    Physical(DamageTypes),
    Magical(DamageTypes),
}

impl Default for DamageClassifications {
    fn default() -> DamageClassifications {
        DamageClassifications::Direct(DamageTypes::Pure)
    }
}

impl Display for DamageClassifications {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DamageClassifications::Direct(damage_type) => write!(f, "direct/{damage_type}"),
            DamageClassifications::Physical(damage_type) => write!(f, "physical/{damage_type}"),
            DamageClassifications::Magical(damage_type) => write!(f, "magical/{damage_type}"),
        }
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
