use core::fmt;
use serde::{Deserialize, Serialize};
use strum_macros::EnumIter;

#[derive(Serialize, Deserialize, Default, EnumIter, Hash, Eq, PartialEq, Copy, Clone, Debug)]
pub enum Jewelries {
    #[default]
    Ring,
    Amulet
}

impl fmt::Display for Jewelries {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Jewelries::Ring => write!(f, "Ring"),
            Jewelries::Amulet => write!(f, "Amulet"),
        }
    }
}
