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
            HeadGears::Cap => write!(f, "Cap"),
            HeadGears::Bandana => write!(f, "Bandana"),
            HeadGears::PaddedCap => write!(f, "Padded Cap"),
            HeadGears::Ribbon => write!(f, "Ribbon"),
            HeadGears::WizardHat => write!(f, "Wizard Hat"),
            HeadGears::Eyepatch => write!(f, "Eyepatch"),
            HeadGears::LeatherHat => write!(f, "Leather Hat"),
            HeadGears::LeatherHelm => write!(f, "Leather Helm"),
            HeadGears::DemonsaurHelm => write!(f, "Demonsaur Helm"),
            HeadGears::Hairpin => write!(f, "Hairpin"),
            HeadGears::Skullcap => write!(f, "Skullcap"),
            HeadGears::Coif => write!(f, "Coif"),
            HeadGears::OhmushellMask => write!(f, "Ohmushell Mask"),
            HeadGears::Circlet => write!(f, "Circlet"),
            HeadGears::Crown => write!(f, "Crown"),
            HeadGears::FullHelm => write!(f, "Full Helm"),
            HeadGears::GreatHelm => write!(f, "Great Helm"),
        }
    }
}
