use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Eq)]
pub enum StatusEffects {
    Poisoned,
    Slowed,
    Weakened,
    Softened,
    Debilitated,
    Regen,
    Refresh,
}
