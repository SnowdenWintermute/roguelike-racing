use crate::combat::hp_change_source_types::PhysicalDamageTypes;
use crate::combat::magical_elements::MagicalElements;
use serde::Deserialize;
use serde::Serialize;

#[derive(Eq, PartialEq, Clone, Debug, Serialize, Deserialize)]
pub enum CombatantTraits {
    HpBioavailabilityPercent(u8),
    MpBioavailabilityPercent(u8),
    ElementalAffinityPercent(MagicalElements, i16),
    Undead,
    PhysicalDamageTypeResistancePercent(PhysicalDamageTypes, i16),
}
