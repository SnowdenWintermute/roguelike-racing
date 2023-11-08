use core::fmt;
use serde::Deserialize;
use serde::Serialize;
use strum_macros::EnumIter;

#[derive(Serialize, Deserialize, Default, EnumIter, Hash, Eq, PartialEq, Copy, Clone, Debug)]
pub enum TwoHandedMeleeWeapons {
    #[default]
    BoStaff,
    Spear,
    Bardiche,
    SplittingMaul,
    Maul,
    BattleAxe,
    Glaive,
    ElementalStaff,
    Trident,
    Halberd,
    GreatAxe,
    GravityHammer,
}

impl fmt::Display for TwoHandedMeleeWeapons {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TwoHandedMeleeWeapons::BoStaff => write!(f, "Staff"),
            TwoHandedMeleeWeapons::Maul => write!(f, "Maul"),
            TwoHandedMeleeWeapons::GravityHammer => write!(f, "Gravity Hammer"),
            TwoHandedMeleeWeapons::SplittingMaul => write!(f, "Splitting Maul"),
            TwoHandedMeleeWeapons::Bardiche => write!(f, "Bardiche"),
            TwoHandedMeleeWeapons::BattleAxe => write!(f, "BattleAxe"),
            TwoHandedMeleeWeapons::GreatAxe => write!(f, "GreatAxe"),
            TwoHandedMeleeWeapons::Glaive => write!(f, "Glaive"),
            TwoHandedMeleeWeapons::Halberd => write!(f, "Halberd"),
            TwoHandedMeleeWeapons::Spear => write!(f, "Spear"),
            TwoHandedMeleeWeapons::Trident => write!(f, "Trident"),
            TwoHandedMeleeWeapons::ElementalStaff => write!(f, "Elemental Staff"),
        }
    }
}
