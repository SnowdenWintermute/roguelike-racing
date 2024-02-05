use crate::combat::magical_elements::MagicalElements;
use serde::Deserialize;
use serde::Serialize;

#[derive(Eq, Hash, PartialEq, Clone, Debug, Serialize, Deserialize)]
pub enum CombatantTraits {
    HpBioavailability,
    ElementalAffinity(MagicalElements, i16),
}
