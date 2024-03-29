use core::fmt;
use serde::Deserialize;
use serde::Serialize;
use strum_macros::EnumIter;

#[derive(Serialize, Deserialize, Default, EnumIter, Hash, Eq, PartialEq, Copy, Clone, Debug)]
pub enum OneHandedMeleeWeapons {
    #[default]
    // PHYSICAL BLUNT
    Stick,
    Mace,
    Morningstar,
    WarHammer,
    // PHYSICAL SLASHING
    ShortSword,
    Blade,
    BroadSword,
    BastardSword,
    // PHYSICAL PIERCING
    Dagger,
    Rapier,
    ShortSpear,
    // PHYSICAL ELEMENTAL
    RuneSword,
    // MAGICAL SLASHING
    EtherBlade,
    IceBlade,
    // FOR MAGES
    MapleWand,
    WillowWand,
    YewWand,
    RoseWand,
}

impl fmt::Display for OneHandedMeleeWeapons {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            OneHandedMeleeWeapons::Mace => write!(f, "Mace"),
            OneHandedMeleeWeapons::Morningstar => write!(f, "Morning Star"),
            OneHandedMeleeWeapons::WarHammer => write!(f, "War Hammer"),
            OneHandedMeleeWeapons::ShortSword => write!(f, "Short Sword"),
            OneHandedMeleeWeapons::Blade => write!(f, "Blade"),
            OneHandedMeleeWeapons::BroadSword => write!(f, "Broad Sword"),
            OneHandedMeleeWeapons::BastardSword => write!(f, "Bastard Sword"),
            OneHandedMeleeWeapons::Stick => write!(f, "Stick"),
            OneHandedMeleeWeapons::RuneSword => write!(f, "Rune Sword"),
            OneHandedMeleeWeapons::Dagger => write!(f, "Dagger"),
            OneHandedMeleeWeapons::Rapier => write!(f, "Rapier"),
            OneHandedMeleeWeapons::ShortSpear => write!(f, "Short Spear"),
            OneHandedMeleeWeapons::EtherBlade => write!(f, "Ether Blade"),
            OneHandedMeleeWeapons::MapleWand => write!(f, "Maple Wand"),
            OneHandedMeleeWeapons::WillowWand => write!(f, "Willow Wand"),
            OneHandedMeleeWeapons::YewWand => write!(f, "Yew Wand"),
            OneHandedMeleeWeapons::RoseWand => write!(f, "Rose Wand"),
            OneHandedMeleeWeapons::IceBlade => write!(f, "Ice Blade"),
        }
    }
}
