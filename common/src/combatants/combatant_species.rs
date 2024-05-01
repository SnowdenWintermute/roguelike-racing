use std::fmt::Display;
use std::fmt::{self};

use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum CombatantSpecies {
    Humanoid,
    Wasp,
    Frog,
    Dragon,
    Skeleton,
    Velociraptor,
}

impl Display for CombatantSpecies {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                CombatantSpecies::Humanoid => "Humanoid",
                CombatantSpecies::Wasp => "Wasp",
                CombatantSpecies::Frog => "Frog",
                CombatantSpecies::Dragon => "Dragon",
                CombatantSpecies::Skeleton => "Skeleton",
                CombatantSpecies::Velociraptor => "Velociraptor",
            }
        )
    }
}
