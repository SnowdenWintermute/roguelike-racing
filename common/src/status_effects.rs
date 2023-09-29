use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum StatusEffects {
    Poisoned,
    Slowed,
    Weakened,
    Softened,
    Debilitated,
    Regen,
    Refresh,
}
