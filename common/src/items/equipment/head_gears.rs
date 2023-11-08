use core::fmt;
use serde::Deserialize;
use serde::Serialize;
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
