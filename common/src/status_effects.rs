use crate::combat::magical_elements::MagicalElements;
use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Eq)]
pub enum StatusEffects {
    Poisoned,
    Slowed,
    Weakened,
    ElementalAffinity(MagicalElements, i16),
    Softened,
    Debilitated,
    Regen,
    Refresh,
}
