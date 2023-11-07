use core::fmt;
use serde::{Deserialize, Serialize};
use strum_macros::EnumIter;

#[derive(Serialize, Deserialize, Default, EnumIter, Hash, Eq, PartialEq, Copy, Clone, Debug)]
pub enum TwoHandedRangedWeapons {
    #[default]
    // PHYSICAL BLUNT
    // PHYSICAL SLASHING
    // PHYSICAL PIERCING
    ShortBow,
    RecurveBow,
    CompositeBow,
    MilitaryBow,
    // PHYSICAL ELEMENTAL
    // MAGICAL SLASHING
    // MAGICAL PIERCING
    EtherBow,
}

impl fmt::Display for TwoHandedRangedWeapons {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TwoHandedRangedWeapons::ShortBow => write!(f, "Short Bow"),
            TwoHandedRangedWeapons::RecurveBow => write!(f, "Recurve Bow"),
            TwoHandedRangedWeapons::CompositeBow => write!(f, "Composite Bow"),
            TwoHandedRangedWeapons::MilitaryBow => write!(f, "Military Bow"),
            TwoHandedRangedWeapons::EtherBow => write!(f, "Ether Bow"),
        }
    }
}
