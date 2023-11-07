use core::fmt;
use serde::{Deserialize, Serialize};
use strum_macros::EnumIter;

#[derive(Serialize, Deserialize, Default, EnumIter, Hash, Eq, PartialEq, Copy, Clone, Debug)]
pub enum Shields {
    #[default]
    MakeshiftBuckler, // small
    WoodenKiteShield, // med
    Buckler,          // small
    Pavise,           // large
    Aspis,            // med
    LanternShield,    // small
    KiteShield,       // med
    TowerShield,      // large
    AncientBuckler,   // small
    GothicShield,     // med
}

impl fmt::Display for Shields {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Shields::MakeshiftBuckler => write!(f, "Makeshift Buckler"),
            Shields::WoodenKiteShield => write!(f, "Wooden KiteShield"),
            Shields::Buckler => write!(f, "Buckler"),
            Shields::Pavise => write!(f, "Pavise"),
            Shields::Aspis => write!(f, "Aspis"),
            Shields::LanternShield => write!(f, "Lantern Shield"),
            Shields::KiteShield => write!(f, "Kite Shield"),
            Shields::TowerShield => write!(f, "Tower Shield"),
            Shields::AncientBuckler => write!(f, "Ancient Buckler"),
            Shields::GothicShield => write!(f, "Aegis"),
        }
    }
}
