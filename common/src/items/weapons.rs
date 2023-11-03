use crate::{app_consts::DEEPEST_FLOOR, primatives::Range};
use serde::{Deserialize, Serialize};
use strum_macros::EnumIter;

#[derive(Debug, EnumIter, Clone, Copy, PartialEq, Serialize, Deserialize, Eq, Default)]
pub enum PhysicalDamageTypes {
    #[default]
    Blunt,
    Slashing,
    Piercing,
}

pub enum Weapons {
    Club,
    Mace,
    Morningstar,
    WarHammer,
    Stick,
    Staff,
    Maul,
    ShortSword,
    Sabre,
    Blade,
    BroadSword,
    BastardSword,
    TwoHandedSword,
    Katana,
    GreatAxe,
    Spear,
    Pike,
    ShortBow,
    HuntersBow,
    LongBow,
    CompositeBow,
    WarBow,
}
