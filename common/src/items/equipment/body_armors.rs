use core::fmt;
use serde::Deserialize;
use serde::Serialize;
use strum_macros::EnumIter;

#[derive(Serialize, Deserialize, Default, EnumIter, Hash, Eq, PartialEq, Copy, Clone, Debug)]
pub enum BodyArmors {
    #[default]
    Rags,
    Cape,
    Cloak,
    Robe,
    Kevlar,
    LeatherArmor,
    HardLeatherArmor,
    StuddedLeatherArmor,
    DemonsaurArmor,
    RingMail,
    ChainMail,
    ScaleMail,
    SplintMail,
    OhmushellMail,
    BreastPlate,
    FieldPlate,
    GothicPlate,
    FullPlate,
    ShardPlate,
}

impl fmt::Display for BodyArmors {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            BodyArmors::Rags => write!(f, "Rags"),
            BodyArmors::Cape => write!(f, "Cape"),
            BodyArmors::Cloak => write!(f, "Cloak"),
            BodyArmors::Robe => write!(f, "Robe"),
            BodyArmors::Kevlar => write!(f, "Kevlar"),
            BodyArmors::LeatherArmor => write!(f, "Leather Armor"),
            BodyArmors::HardLeatherArmor => write!(f, "Hard Leather Armor"),
            BodyArmors::StuddedLeatherArmor => write!(f, "Studded Leather Armor"),
            BodyArmors::DemonsaurArmor => write!(f, "Demonsaur Armor"),
            BodyArmors::RingMail => write!(f, "Ring Mail"),
            BodyArmors::ChainMail => write!(f, "Chain Mail"),
            BodyArmors::ScaleMail => write!(f, "Scale Mail"),
            BodyArmors::SplintMail => write!(f, "Splint Mail"),
            BodyArmors::OhmushellMail => write!(f, "Ohmushell Mail"),
            BodyArmors::BreastPlate => write!(f, "Breast Plate"),
            BodyArmors::FieldPlate => write!(f, "Field Plate"),
            BodyArmors::GothicPlate => write!(f, "Gothic Plate"),
            BodyArmors::FullPlate => write!(f, "Full Plate"),
            BodyArmors::ShardPlate => write!(f, "Shard Plate"),
        }
    }
}
