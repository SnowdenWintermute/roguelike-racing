use core::fmt;
use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum MagicalElements {
    Fire,
    Ice,
    Lightning,
    Water,
    Earth,
    Wind,
    Dark,
    Light,
}

impl fmt::Display for MagicalElements {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MagicalElements::Fire => write!(f, "Fire"),
            MagicalElements::Ice => write!(f, "Ice"),
            MagicalElements::Lightning => write!(f, "Lightning"),
            MagicalElements::Water => write!(f, "Water"),
            MagicalElements::Earth => write!(f, "Earth"),
            MagicalElements::Wind => write!(f, "Wind"),
            MagicalElements::Dark => write!(f, "Dark"),
            MagicalElements::Light => write!(f, "Light"),
        }
    }
}
